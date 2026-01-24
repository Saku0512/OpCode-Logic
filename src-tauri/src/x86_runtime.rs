use crate::vm::{Register, Syntax, VmState};
use crate::x86_asm::assemble_x86_64;

use std::collections::{HashMap, VecDeque};

use unicorn_engine::unicorn_const::{Arch, Mode, Prot, RegisterX86, X86Insn};
use unicorn_engine::Unicorn;

const CODE_BASE: u64 = 0x0010_0000;
const BSS_BASE: u64 = 0x0020_0000;
const STACK_BASE: u64 = 0x0030_0000;
const STACK_SIZE: u64 = 0x0020_0000; // 2MB

const PAGE_SIZE: u64 = 0x1000;

fn align_up(x: u64, align: u64) -> u64 {
    if x.is_multiple_of(align) {
        x
    } else {
        x + (align - (x % align))
    }
}

#[derive(Default)]
struct RuntimeData {
    input: VecDeque<i64>,
    output: Vec<i64>,
    exited: bool,
    error: Option<String>,
}

pub struct RunResult {
    pub state: VmState,
    pub execution_log: Vec<String>,
}

#[derive(Default)]
struct BssLayout {
    total_size: u64,
    labels: HashMap<String, u64>,
}

fn parse_bss_layout(code: &str) -> BssLayout {
    let mut layout = BssLayout::default();
    let mut in_bss = false;
    let mut offset = 0u64;

    for raw in code.lines() {
        let line = raw.split(&[';', '#'][..]).next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let lower = line.to_lowercase();
        if lower.starts_with("section ") {
            in_bss = lower.contains(".bss");
            continue;
        }
        if !in_bss {
            continue;
        }

        // Accept:
        //  - "buf: resb 16"
        //  - "buf resb 16"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let label_raw = parts[0].trim_end_matches(':');
        let op = parts[1].to_lowercase();
        if op != "resb" {
            continue;
        }
        if let Ok(size) = parts[2].parse::<u64>() {
            layout
                .labels
                .insert(label_raw.to_string(), BSS_BASE + offset);
            offset += size;
        }
    }

    layout.total_size = offset;
    layout
}

fn extract_text_section(code: &str) -> String {
    let mut in_text = false;
    let mut out = String::new();

    for raw in code.lines() {
        let line = raw.split(&[';', '#'][..]).next().unwrap_or("").trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let lower = line.trim().to_lowercase();
        if lower.starts_with("section ") {
            in_text = lower.contains(".text");
            continue;
        }
        if ["global", "extern", "default", "bits"]
            .iter()
            .any(|k| lower.starts_with(k))
        {
            continue;
        }
        if !in_text {
            continue;
        }

        out.push_str(line);
        out.push('\n');
    }

    // If there was no explicit .text, fall back to all non-.bss lines
    if out.trim().is_empty() {
        for raw in code.lines() {
            let line = raw.split(&[';', '#'][..]).next().unwrap_or("").trim_end();
            if line.trim().is_empty() {
                continue;
            }
            let lower = line.trim().to_lowercase();
            if lower.starts_with("section ") && lower.contains(".bss") {
                // skip until next section
                continue;
            }
            if lower.contains(" resb ") || lower.ends_with(" resb") || lower.contains(" resb\t") {
                continue;
            }
            if ["global", "extern", "default", "bits"]
                .iter()
                .any(|k| lower.starts_with(k))
            {
                continue;
            }
            out.push_str(line);
            out.push('\n');
        }
    }

    out
}

fn preprocess_text(code: &str, syntax: &Syntax, bss_labels: &HashMap<String, u64>) -> String {
    // Minimal source-to-source transforms to support existing tutorial syntax:
    // - Replace immediate usage: `mov rsi, buf` -> `mov rsi, 0xADDR`
    // - Replace memory label usage in Intel style: `[buf + r8]` -> `mov r15, 0xADDR` + `[r15 + r8]`
    // - Replace AT&T displacement usage: `buf(%rsi)` -> `0xADDR(%rsi)` and `$buf` -> `$0xADDR`
    //
    // NOTE: This uses r15 as a scratch register for Intel memory operands.
    let mut out = String::new();
    let text = extract_text_section(code);

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let line = normalize_decimal_literals(line);

        // Keystone label fixups do not handle `loop <label>` well when we rewrite labels
        // into absolute immediates. Rewrite it into `dec rcx; jnz <label>`.
        let lower = line.trim_start().to_lowercase();
        if lower.starts_with("loop ") {
            let target = line.trim_start()[4..].trim();
            match syntax {
                Syntax::Intel => {
                    out.push_str("dec rcx\n");
                    out.push_str(&format!("jnz {}\n", target));
                }
                Syntax::Att => {
                    out.push_str("decq %rcx\n");
                    out.push_str(&format!("jnz {}\n", target));
                }
            }
            continue;
        }

        match syntax {
            Syntax::Intel => {
                if let Some((before, inside, after)) = split_first_bracket(&line) {
                    if let Some((label, rest)) = take_leading_ident(inside) {
                        if let Some(addr) = bss_labels.get(label) {
                            // Insert scratch-load and rewrite [label ...] -> [r15 ...]
                            out.push_str(&format!("mov r15, 0x{:x}\n", addr));
                            out.push_str(before);
                            out.push('[');
                            out.push_str("r15");
                            out.push_str(rest);
                            out.push(']');
                            out.push_str(after);
                            out.push('\n');
                            continue;
                        }
                    }
                }

                // Immediate substitution for bss labels used outside brackets
                let mut replaced = line.to_string();
                for (name, addr) in bss_labels {
                    replaced = replace_ident(&replaced, name, &format!("0x{:x}", addr));
                }
                out.push_str(&replaced);
                out.push('\n');
            }
            Syntax::Att => {
                let mut replaced = line.to_string();
                for (name, addr) in bss_labels {
                    // `$buf` -> `$0x...`
                    replaced = replaced.replace(&format!("${}", name), &format!("$0x{:x}", addr));
                    // `buf(` -> `0x...(`  (displacement before parens)
                    replaced = replaced.replace(&format!("{}(", name), &format!("0x{:x}(", addr));
                }
                out.push_str(&replaced);
                out.push('\n');
            }
        }
    }

    out
}

fn normalize_decimal_literals(line: &str) -> String {
    // Keystone's numeric parsing for x86 syntaxes tends to treat bare numbers as hex.
    // Our learning content uses decimal, so rewrite `123` -> `0x7b` and `-5` -> `-0x5`.
    //
    // Rules:
    // - Only rewrite standalone integer tokens matching /^-?\d+$/.
    // - Do not touch 0x/0b/0o prefixed numbers.
    // - Do not touch numbers inside quotes (e.g. 'A').
    let mut out = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let mut i = 0usize;
    let mut in_single_quote = false;

    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '\'' {
            in_single_quote = !in_single_quote;
            out.push(c);
            i += 1;
            continue;
        }
        if in_single_quote {
            out.push(c);
            i += 1;
            continue;
        }

        let boundary = |ch: Option<char>| match ch {
            None => true,
            Some(cc) => cc.is_ascii_whitespace() || ",[]()+-*/:\t$".contains(cc),
        };

        let prev = if i == 0 {
            None
        } else {
            Some(bytes[i - 1] as char)
        };
        // If we are at a boundary and see a prefixed literal (0x/0b/0o),
        // copy it verbatim. This avoids turning `0x20` into `0x0x20`.
        if boundary(prev) {
            // 0x... / 0b... / 0o...
            if i + 1 < bytes.len() && bytes[i] == b'0' {
                let p = bytes[i + 1];
                if p == b'x' || p == b'X' || p == b'b' || p == b'B' || p == b'o' || p == b'O' {
                    // Copy prefix
                    out.push('0');
                    out.push(p as char);
                    i += 2;
                    // Copy digits
                    while i < bytes.len() {
                        let cc = bytes[i] as char;
                        let ok = match p {
                            b'x' | b'X' => cc.is_ascii_hexdigit() || cc == '_',
                            b'b' | b'B' => cc == '0' || cc == '1' || cc == '_',
                            b'o' | b'O' => (cc.is_ascii_digit() && cc <= '7') || cc == '_',
                            _ => false,
                        };
                        if ok {
                            out.push(cc);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    continue;
                }
            }
            // -0x... / -0b... / -0o...
            if i + 2 < bytes.len() && bytes[i] == b'-' && bytes[i + 1] == b'0' {
                let p = bytes[i + 2];
                if p == b'x' || p == b'X' || p == b'b' || p == b'B' || p == b'o' || p == b'O' {
                    out.push('-');
                    out.push('0');
                    out.push(p as char);
                    i += 3;
                    while i < bytes.len() {
                        let cc = bytes[i] as char;
                        let ok = match p {
                            b'x' | b'X' => cc.is_ascii_hexdigit() || cc == '_',
                            b'b' | b'B' => cc == '0' || cc == '1' || cc == '_',
                            b'o' | b'O' => (cc.is_ascii_digit() && cc <= '7') || cc == '_',
                            _ => false,
                        };
                        if ok {
                            out.push(cc);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    continue;
                }
            }
        }

        let next = if i + 1 < bytes.len() {
            Some(bytes[i + 1] as char)
        } else {
            None
        };

        let starts_number = (c.is_ascii_digit() && boundary(prev))
            || (c == '-' && next.is_some_and(|n| n.is_ascii_digit()) && boundary(prev));

        if !starts_number {
            out.push(c);
            i += 1;
            continue;
        }

        let start = i;
        i += 1;
        while i < bytes.len() {
            let cc = bytes[i] as char;
            if cc.is_ascii_digit() {
                i += 1;
            } else {
                break;
            }
        }
        let token = &line[start..i];

        // Already prefixed? (e.g. 0x12) — don't rewrite.
        if token.starts_with("0x")
            || token.starts_with("-0x")
            || token.starts_with("0b")
            || token.starts_with("-0b")
        {
            out.push_str(token);
            continue;
        }

        if let Ok(val) = token.parse::<i64>() {
            if val < 0 {
                out.push_str(&format!("-0x{:x}", -val));
            } else {
                out.push_str(&format!("0x{:x}", val));
            }
        } else {
            out.push_str(token);
        }
    }

    out
}

fn split_first_bracket(line: &str) -> Option<(&str, &str, &str)> {
    let start = line.find('[')?;
    let end = line[start + 1..].find(']')? + start + 1;
    let before = &line[..start];
    let inside = &line[start + 1..end];
    let after = &line[end + 1..];
    Some((before, inside, after))
}

fn take_leading_ident(s: &str) -> Option<(&str, &str)> {
    let t = s.trim_start();
    let mut chars = t.char_indices();
    let (first_i, first_c) = chars.next()?;
    if !(first_c.is_ascii_alphabetic() || first_c == '_' || first_c == '.') {
        return None;
    }
    let mut end = first_i + first_c.len_utf8();
    for (i, c) in chars {
        if c.is_ascii_alphanumeric() || c == '_' || c == '.' {
            end = i + c.len_utf8();
        } else {
            break;
        }
    }
    let ident = &t[..end];
    let rest = &t[end..];
    Some((ident, rest))
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.'
}

fn replace_ident(input: &str, ident: &str, replacement: &str) -> String {
    // Boundary-aware identifier replacement.
    let mut out = String::with_capacity(input.len());
    let mut i = 0usize;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        let c = bytes[i] as char;
        if !(c.is_ascii_alphabetic() || c == '_' || c == '.') {
            out.push(c);
            i += 1;
            continue;
        }

        let start = i;
        i += 1;
        while i < bytes.len() {
            let cc = bytes[i] as char;
            if is_ident_char(cc) {
                i += 1;
            } else {
                break;
            }
        }

        let token = &input[start..i];
        if token == ident {
            out.push_str(replacement);
        } else {
            out.push_str(token);
        }
    }
    out
}

pub fn run_x86_64(
    code: &str,
    syntax: Syntax,
    input: Vec<i64>,
    max_instructions: usize,
) -> Result<RunResult, String> {
    let mut execution_log: Vec<String> = Vec::new();

    let bss = parse_bss_layout(code);
    let bss_size = align_up(bss.total_size.max(512), PAGE_SIZE);

    let preprocessed = preprocess_text(code, &syntax, &bss.labels);
    execution_log.push("Assembling...".to_string());

    let assembled = assemble_x86_64(&preprocessed, syntax.clone(), CODE_BASE)?;
    let code_size = align_up(assembled.bytes.len().max(1) as u64, PAGE_SIZE);

    // Set up Unicorn
    let data = RuntimeData {
        input: VecDeque::from(input),
        output: Vec::new(),
        exited: false,
        error: None,
    };

    let mut emu: Unicorn<RuntimeData> =
        Unicorn::new_with_data(Arch::X86, Mode::MODE_64, data).map_err(|e| format!("{e:?}"))?;

    emu.mem_map(CODE_BASE, code_size, Prot::ALL)
        .map_err(|e| format!("mem_map code failed: {e:?}"))?;
    emu.mem_map(BSS_BASE, bss_size, Prot::ALL)
        .map_err(|e| format!("mem_map bss failed: {e:?}"))?;
    emu.mem_map(STACK_BASE, STACK_SIZE, Prot::ALL)
        .map_err(|e| format!("mem_map stack failed: {e:?}"))?;

    emu.mem_write(CODE_BASE, &assembled.bytes)
        .map_err(|e| format!("mem_write code failed: {e:?}"))?;

    // Initialize registers
    emu.reg_write(RegisterX86::RSP, STACK_BASE + STACK_SIZE - 8)
        .map_err(|e| format!("reg_write RSP failed: {e:?}"))?;

    let entry = assembled.labels.get("_start").copied().unwrap_or(CODE_BASE);
    emu.reg_write(RegisterX86::RIP, entry)
        .map_err(|e| format!("reg_write RIP failed: {e:?}"))?;

    // Syscall hook
    emu.add_insn_sys_hook(X86Insn::SYSCALL, CODE_BASE, CODE_BASE + code_size, |uc| {
        let rax = uc.reg_read(RegisterX86::RAX).unwrap_or(0);
        match rax {
            0 => {
                // read(fd=rdi, buf=rsi, count=rdx) - we ignore fd and always read from input queue
                let count = uc.reg_read(RegisterX86::RDX).unwrap_or(0) as usize;
                let addr = uc.reg_read(RegisterX86::RSI).unwrap_or(0);
                let mut read = 0usize;
                for i in 0..count {
                    if let Some(v) = uc.get_data_mut().input.pop_front() {
                        let b = (v & 0xff) as u8;
                        let _ = uc.mem_write(addr + i as u64, &[b]);
                        read += 1;
                    } else {
                        break;
                    }
                }
                let _ = uc.reg_write(RegisterX86::RAX, read as u64);
            }
            1 => {
                // write(fd=rdi, buf=rsi, count=rdx)
                let count = uc.reg_read(RegisterX86::RDX).unwrap_or(0) as usize;
                let addr = uc.reg_read(RegisterX86::RSI).unwrap_or(0);
                let mut buf = vec![0u8; count];
                if uc.mem_read(addr, &mut buf).is_ok() {
                    for b in buf {
                        let val = if (b & 0x80) != 0 {
                            (b as i8) as i64
                        } else {
                            b as i64
                        };
                        uc.get_data_mut().output.push(val);
                    }
                    let _ = uc.reg_write(RegisterX86::RAX, count as u64);
                } else {
                    uc.get_data_mut().error = Some("sys_write: invalid address".to_string());
                    let _ = uc.emu_stop();
                }
            }
            60 => {
                uc.get_data_mut().exited = true;
                let _ = uc.emu_stop();
            }
            other => {
                uc.get_data_mut().error = Some(format!("Unknown syscall: {}", other));
                let _ = uc.emu_stop();
            }
        }
    })
    .map_err(|e| format!("add_insn_sys_hook failed: {e:?}"))?;

    // Run
    let start = entry;
    let until = CODE_BASE + code_size;
    let run_res = emu.emu_start(start, until, 0, max_instructions);

    if let Err(e) = run_res {
        emu.get_data_mut().error = Some(format!("Emulation error: {e:?}"));
    } else if !emu.get_data().exited && emu.get_data().error.is_none() {
        // If we stopped without exit and without explicit error, we likely hit instruction limit.
        emu.get_data_mut().error = Some("Instruction limit reached".to_string());
    }

    // Build VmState
    let mut registers = HashMap::new();
    let reg_map: &[(Register, RegisterX86)] = &[
        (Register::RAX, RegisterX86::RAX),
        (Register::RBX, RegisterX86::RBX),
        (Register::RCX, RegisterX86::RCX),
        (Register::RDX, RegisterX86::RDX),
        (Register::RSI, RegisterX86::RSI),
        (Register::RDI, RegisterX86::RDI),
        (Register::RSP, RegisterX86::RSP),
        (Register::RBP, RegisterX86::RBP),
        (Register::R8, RegisterX86::R8),
        (Register::R9, RegisterX86::R9),
        (Register::R10, RegisterX86::R10),
        (Register::R11, RegisterX86::R11),
        (Register::R12, RegisterX86::R12),
        (Register::R13, RegisterX86::R13),
        (Register::R14, RegisterX86::R14),
        (Register::R15, RegisterX86::R15),
    ];

    for (k, r) in reg_map {
        let v = emu.reg_read(*r).unwrap_or(0) as i64;
        registers.insert(*k, v);
    }

    let rflags = emu.reg_read(RegisterX86::EFLAGS).unwrap_or(0);
    let zf = (rflags & (1 << 6)) != 0;
    let sf = (rflags & (1 << 7)) != 0;

    let rip = emu.reg_read(RegisterX86::RIP).unwrap_or(entry);
    let pc = if rip >= CODE_BASE {
        (rip - CODE_BASE) as usize
    } else {
        0
    };

    // Return first 512 bytes from BSS region for UI
    let mut mem512 = vec![0u8; 512];
    let _ = emu.mem_read(BSS_BASE, &mut mem512);

    let state = VmState {
        registers,
        zf,
        sf,
        pc,
        output: emu.get_data().output.clone(),
        stack: Vec::new(),
        memory: mem512,
        input_remaining: emu.get_data().input.len(),
        finished: emu.get_data().exited || emu.get_data().error.is_some(),
        exited: emu.get_data().exited,
        error: emu.get_data().error.clone(),
    };

    Ok(RunResult {
        state,
        execution_log,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_syscall_read_write_exit_intel() {
        let code = r#"
section .bss
    buf resb 16

section .text
    global _start

_start:
    mov rax, 0
    mov rdi, 0
    mov rsi, buf
    mov rdx, 1
    syscall

    mov rdx, rax
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    syscall

    mov rax, 60
    xor rdi, rdi
    syscall
"#;
        let res = run_x86_64(code, Syntax::Intel, vec![123], 10_000).unwrap();
        assert_eq!(res.state.output, vec![123]);
        assert!(res.state.exited, "VmState: {:?}", res.state);
        assert!(res.state.error.is_none(), "VmState: {:?}", res.state);
    }

    #[test]
    fn runs_syscall_read_write_exit_att() {
        let code = r#"
section .bss
    buf resb 16

section .text
    global _start

_start:
    movq $0, %rax
    movq $0, %rdi
    movq $buf, %rsi
    movq $1, %rdx
    syscall

    movq %rax, %rdx
    movq $1, %rax
    movq $1, %rdi
    movq $buf, %rsi
    syscall

    movq $60, %rax
    xorq %rdi, %rdi
    syscall
"#;
        let res = run_x86_64(code, Syntax::Att, vec![42], 10_000).unwrap();
        assert_eq!(res.state.output, vec![42]);
        assert!(res.state.exited, "VmState: {:?}", res.state);
        assert!(res.state.error.is_none(), "VmState: {:?}", res.state);
    }
}
