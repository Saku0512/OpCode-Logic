use crate::vm::Syntax;

use keystone_engine::{Arch, Keystone, KeystoneOutput, Mode, OptionType, OptionValue};
use std::collections::HashMap;

pub struct AssembleResult {
    pub bytes: Vec<u8>,
    pub labels: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
enum Entry {
    Label(String),
    Inst(String),
}

fn strip_comment(line: &str) -> &str {
    line.split(&[';', '#'][..]).next().unwrap_or("").trim()
}

fn is_label_def(line: &str) -> Option<&str> {
    let t = line.trim();
    t.strip_suffix(':').map(|s| s.trim()).filter(|s| !s.is_empty())
}

fn is_ignorable_directive(line: &str) -> bool {
    let lower = line.trim().to_lowercase();
    lower.starts_with("section ")
        || lower.starts_with("global ")
        || lower.starts_with("extern ")
        || lower.starts_with("default ")
        || lower.starts_with("bits ")
}

fn parse_entries(code: &str) -> Vec<Entry> {
    let mut out = Vec::new();
    for raw in code.lines() {
        let line = strip_comment(raw);
        if line.is_empty() {
            continue;
        }
        if is_ignorable_directive(line) {
            continue;
        }
        if let Some(lbl) = is_label_def(line) {
            out.push(Entry::Label(lbl.to_string()));
            continue;
        }
        out.push(Entry::Inst(line.to_string()));
    }
    out
}

fn init_engine(syntax: Syntax) -> Result<Keystone, String> {
    let engine =
        Keystone::new(Arch::X86, Mode::MODE_64).map_err(|e| format!("Keystone init: {e:?}"))?;

    match syntax {
        // Our project uses NASM-flavored Intel syntax (directives, label style).
        // Keystone's SYNTAX_NASM matches this best.
        Syntax::Intel => engine
            .option(OptionType::SYNTAX, OptionValue::SYNTAX_NASM)
            .map_err(|e| format!("Keystone set NASM syntax: {e:?}"))?,
        Syntax::Att => engine
            .option(OptionType::SYNTAX, OptionValue::SYNTAX_ATT)
            .map_err(|e| format!("Keystone set AT&T syntax: {e:?}"))?,
    }
    Ok(engine)
}

fn replace_symbols_with_addrs(
    line: &str,
    labels: &HashMap<String, u64>,
    known_labels: &HashMap<String, ()>,
) -> String {
    // Replace occurrences of label identifiers with hex immediates.
    // This is a minimal, boundary-aware replacer to avoid clobbering registers/keywords.
    let bytes = line.as_bytes();
    let mut i = 0usize;
    let mut out = String::with_capacity(line.len());

    while i < bytes.len() {
        let c = bytes[i] as char;
        let is_ident_start = c.is_ascii_alphabetic() || c == '_' || c == '.';
        if !is_ident_start {
            out.push(c);
            i += 1;
            continue;
        }

        let start = i;
        i += 1;
        while i < bytes.len() {
            let cc = bytes[i] as char;
            if cc.is_ascii_alphanumeric() || cc == '_' || cc == '.' {
                i += 1;
            } else {
                break;
            }
        }
        let ident = &line[start..i];
        if let Some(addr) = labels.get(ident) {
            out.push_str(&format!("0x{:x}", addr));
        } else if known_labels.contains_key(ident) {
            // Allow first-pass assembly before we know final label addresses.
            out.push_str("0x0");
        } else {
            out.push_str(ident);
        }
    }

    out
}

fn asm_one(engine: &Keystone, inst: &str, addr: u64) -> Result<KeystoneOutput, String> {
    engine
        .asm(inst.to_string(), addr)
        .map_err(|e| format!("Assemble failed at 0x{addr:x}: `{inst}`: {e:?}"))
}

/// Assemble x86_64 with a minimal label-resolver.
///
/// Keystone label handling can be unreliable for some inputs, so we:
/// - parse `label:` lines ourselves
/// - iteratively compute label addresses and rewrite label references into absolute hex immediates
/// - assemble line-by-line with correct per-instruction address
pub fn assemble_x86_64(code: &str, syntax: Syntax, base_addr: u64) -> Result<AssembleResult, String> {
    let engine = init_engine(syntax)?;
    let entries = parse_entries(code);

    let mut labels: HashMap<String, u64> = HashMap::new();
    let mut last_labels: HashMap<String, u64> = HashMap::new();
    let mut known_labels: HashMap<String, ()> = HashMap::new();

    for e in &entries {
        if let Entry::Label(name) = e {
            known_labels.insert(name.clone(), ());
        }
    }

    // Iterate to stabilize label locations in case of short/near encoding selection.
    for _ in 0..6 {
        labels.clear();

        let mut pc = base_addr;
        for e in &entries {
            match e {
                Entry::Label(name) => {
                    labels.insert(name.clone(), pc);
                }
                Entry::Inst(raw) => {
                    let inst = replace_symbols_with_addrs(raw, &last_labels, &known_labels);
                    let out = asm_one(&engine, &inst, pc)?;
                    pc = pc.wrapping_add(out.bytes.len() as u64);
                }
            }
        }

        if labels == last_labels {
            break;
        }

        // Next iteration uses the newly computed label locations.
        last_labels = labels.clone();
    }

    // Final assembly
    let mut bytes = Vec::new();
    let mut pc = base_addr;
    for e in &entries {
        match e {
            Entry::Label(_) => {}
            Entry::Inst(raw) => {
                let inst = replace_symbols_with_addrs(raw, &labels, &known_labels);
                let out = asm_one(&engine, &inst, pc)?;
                pc = pc.wrapping_add(out.bytes.len() as u64);
                bytes.extend_from_slice(&out.bytes);
            }
        }
    }

    Ok(AssembleResult { bytes, labels })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembles_intel_jge_with_label() {
        let code = r#"
            _start:
                mov rax, 1
            loop:
                cmp rax, 0
                jge loop
        "#;
        let res = assemble_x86_64(code, Syntax::Intel, 0x1000).unwrap();
        assert!(!res.bytes.is_empty());
    }

    #[test]
    fn assembles_att_jge_with_label() {
        let code = r#"
            _start:
                movq $1, %rax
            loop:
                cmpq $0, %rax
                jge loop
        "#;
        let res = assemble_x86_64(code, Syntax::Att, 0x1000).unwrap();
        assert!(!res.bytes.is_empty());
    }
}

