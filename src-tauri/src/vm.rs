use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Register {
    RAX, RBX, RCX, RDX, RSI, RDI, RSP, RBP,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    Reg(Register),
    Imm(i64),
    Label(String),
    MemReg(Register),     // [RAX]
    MemLabel(String),    // [buf]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Syntax {
    Intel,
    Att,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    MOV(Operand, Operand),
    ADD(Operand, Operand),
    SUB(Operand, Operand),
    INC(Operand),
    DEC(Operand),
    XOR(Operand, Operand),
    CMP(Operand, Operand),
    TEST(Operand, Operand),
    JMP(String),
    JZ(String),
    JNZ(String),
    JS(String),
    PUSH(Operand),
    POP(Operand),
    IN(Operand),
    OUT(Operand),
    SYSCALL,
    RET,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VmState {
    pub registers: HashMap<Register, i64>,
    pub zf: bool,
    pub sf: bool,
    pub pc: usize,
    pub output: Vec<i64>,
    pub stack: Vec<i64>,
    pub memory: Vec<u8>, // Visualization of memory might be too large, but necessary for state
    pub input_remaining: usize,
    pub finished: bool,
    pub error: Option<String>,
}

pub struct VM {
    registers: HashMap<Register, i64>,
    zf: bool,
    sf: bool,
    pc: usize,
    program: Vec<Instruction>,
    labels: HashMap<String, usize>,
    data_labels: HashMap<String, usize>, // label -> memory address
    memory: Vec<u8>,
    stack: Vec<i64>,
    input_queue: VecDeque<i64>,
    output_queue: Vec<i64>,
    error: Option<String>,
    finished: bool,
}

const MEMORY_SIZE: usize = 65536; // 64KB

impl VM {
    pub fn new(program: Vec<Instruction>, labels: HashMap<String, usize>, data_labels: HashMap<String, usize>, input: Vec<i64>) -> Self {
        let mut registers = HashMap::new();
        for r in [
            Register::RAX, Register::RBX, Register::RCX, Register::RDX,
            Register::RSI, Register::RDI, Register::RSP, Register::RBP,
            Register::R8,  Register::R9,  Register::R10, Register::R11,
            Register::R12, Register::R13, Register::R14, Register::R15
        ] {
            registers.insert(r, 0);
        }
        
        VM {
            registers,
            zf: false,
            sf: false,
            pc: 0,
            program,
            labels,
            data_labels,
            memory: vec![0; MEMORY_SIZE],
            stack: Vec::new(),
            input_queue: VecDeque::from(input),
            output_queue: Vec::new(),
            error: None,
            finished: false,
        }
    }

    pub fn set_register(&mut self, reg: Register, val: i64) {
        self.registers.insert(reg, val);
    }

    pub fn get_register(&self, reg: Register) -> i64 {
        *self.registers.get(&reg).unwrap_or(&0)
    }

    pub fn get_state(&self) -> VmState {
        VmState {
            registers: self.registers.clone(),
            zf: self.zf,
            sf: self.sf,
            pc: self.pc,
            output: self.output_queue.clone(),
            stack: self.stack.clone(),
            memory: self.memory[0..512].to_vec(), // Only return first 512 bytes for UI performance
            input_remaining: self.input_queue.len(),
            finished: self.finished || self.pc >= self.program.len() || self.error.is_some(),
            error: self.error.clone(),
        }
    }

    fn get_value(&self, op: &Operand) -> Result<i64, String> {
        match op {
            Operand::Reg(r) => Ok(self.get_register(*r)),
            Operand::Imm(val) => Ok(*val),
            Operand::Label(l) => {
                // If it's a label in an instruction like "mov rsi, buf", we mean the address
                if let Some(&addr) = self.data_labels.get(l) {
                    Ok(addr as i64)
                } else if let Some(&addr) = self.labels.get(l) {
                    Ok(addr as i64)
                } else {
                    Err(format!("Symbol not found: {}", l))
                }
            },
            Operand::MemReg(r) => {
                let addr = self.get_register(*r) as usize;
                if addr + 8 <= self.memory.len() {
                    let bytes = &self.memory[addr..addr+8];
                    Ok(i64::from_le_bytes(bytes.try_into().unwrap()))
                } else {
                    Err(format!("Segmentation fault at address 0x{:x}", addr))
                }
            },
            Operand::MemLabel(l) => {
                if let Some(&addr) = self.data_labels.get(l) {
                    if addr + 8 <= self.memory.len() {
                        let bytes = &self.memory[addr..addr+8];
                        Ok(i64::from_le_bytes(bytes.try_into().unwrap()))
                    } else {
                        Err(format!("Segmentation fault at symbol {}", l))
                    }
                } else {
                    Err(format!("Memory label not found: {}", l))
                }
            }
        }
    }

    fn set_value(&mut self, op: &Operand, val: i64) -> Result<(), String> {
        match op {
            Operand::Reg(r) => {
                self.registers.insert(*r, val);
                Ok(())
            },
            Operand::MemReg(r) => {
                let addr = self.get_register(*r) as usize;
                if addr + 1 <= self.memory.len() {
                    // Supporting partial writes is complex, but for now let's do byte if it's a small value??
                    // Real x86_64 mov [buf], rax writes 8 bytes.
                    if addr + 8 <= self.memory.len() {
                        let bytes = val.to_le_bytes();
                        self.memory[addr..addr+8].copy_from_slice(&bytes);
                        Ok(())
                    } else {
                        self.memory[addr] = (val & 0xFF) as u8;
                        Ok(())
                    }
                } else {
                    Err(format!("Segmentation fault (store) at 0x{:x}", addr))
                }
            },
            Operand::MemLabel(l) => {
                if let Some(&addr) = self.data_labels.get(l) {
                    if addr + 8 <= self.memory.len() {
                        let bytes = val.to_le_bytes();
                        self.memory[addr..addr+8].copy_from_slice(&bytes);
                        Ok(())
                    } else {
                        self.memory[addr] = (val & 0xFF) as u8;
                        Ok(())
                    }
                } else {
                    Err(format!("Symbol not found: {}", l))
                }
            },
            _ => Err("Invalid destination for store".to_string()),
        }
    }

    fn update_flags(&mut self, val: i64) {
        self.zf = val == 0;
        self.sf = val < 0;
    }

    pub fn step(&mut self) -> bool {
        if self.finished || self.pc >= self.program.len() || self.error.is_some() {
            return false;
        }

        let inst = self.program[self.pc].clone();
        let mut next_pc = self.pc + 1;

        match inst {
            Instruction::MOV(dest, src) => {
                match self.get_value(&src) {
                    Ok(val) => {
                        if let Err(e) = self.set_value(&dest, val) {
                            self.error = Some(e);
                        }
                    },
                    Err(e) => { self.error = Some(e); }
                }
            },
            Instruction::ADD(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1.wrapping_add(v2);
                    let _ = self.set_value(&dest, res);
                    self.update_flags(res);
                } else { self.error = Some("Invalid operands for ADD".to_string()); }
            },
            Instruction::SUB(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1.wrapping_sub(v2);
                    let _ = self.set_value(&dest, res);
                    self.update_flags(res);
                } else { self.error = Some("Invalid operands for SUB".to_string()); }
            },
            Instruction::INC(op) => {
                if let Ok(val) = self.get_value(&op) {
                    let res = val.wrapping_add(1);
                    let _ = self.set_value(&op, res);
                    self.update_flags(res);
                } else { self.error = Some("Invalid operand for INC".to_string()); }
            },
            Instruction::DEC(op) => {
                if let Ok(val) = self.get_value(&op) {
                    let res = val.wrapping_sub(1);
                    let _ = self.set_value(&op, res);
                    self.update_flags(res);
                } else { self.error = Some("Invalid operand for DEC".to_string()); }
            },
            Instruction::XOR(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1 ^ v2;
                    let _ = self.set_value(&dest, res);
                    self.update_flags(res);
                } else { self.error = Some("Invalid operands for XOR".to_string()); }
            },
            Instruction::CMP(op1, op2) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&op1), self.get_value(&op2)) {
                    let res = v1.wrapping_sub(v2);
                    self.update_flags(res);
                } else { self.error = Some("Invalid operands for CMP".to_string()); }
            },
            Instruction::TEST(op1, op2) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&op1), self.get_value(&op2)) {
                    let res = v1 & v2;
                    self.update_flags(res);
                } else { self.error = Some("Invalid operands for TEST".to_string()); }
            },
            Instruction::PUSH(op) => {
                if let Ok(val) = self.get_value(&op) {
                    self.stack.push(val);
                } else { self.error = Some("Invalid source for PUSH".to_string()); }
            },
            Instruction::POP(op) => {
                if let Some(val) = self.stack.pop() {
                    if let Err(e) = self.set_value(&op, val) {
                        self.error = Some(e);
                    }
                } else { self.error = Some("Stack underflow".to_string()); }
            },
            Instruction::IN(op) => {
                if let Some(val) = self.input_queue.pop_front() {
                    if let Err(e) = self.set_value(&op, val) {
                        self.error = Some(e);
                    }
                } else { self.error = Some("Input buffer empty".to_string()); }
            },
            Instruction::OUT(op) => {
                if let Ok(val) = self.get_value(&op) {
                    self.output_queue.push(val);
                } else { self.error = Some("Invalid source for OUT".to_string()); }
            },
            Instruction::SYSCALL => {
                let rax = self.get_register(Register::RAX);
                match rax {
                    0 => { // sys_read
                        let count = self.get_register(Register::RDX) as usize;
                        let addr = self.get_register(Register::RSI) as usize;
                        let mut read_bytes = 0;
                        for i in 0..count {
                            if let Some(val) = self.input_queue.pop_front() {
                                if addr + i < self.memory.len() {
                                    self.memory[addr + i] = (val & 0xFF) as u8;
                                    read_bytes += 1;
                                }
                            } else { break; }
                        }
                        self.registers.insert(Register::RAX, read_bytes as i64);
                    },
                    1 => { // sys_write
                        let count = self.get_register(Register::RDX) as usize;
                        let addr = self.get_register(Register::RSI) as usize;
                        let mut written_bytes = 0;
                        for i in 0..count {
                            if addr + i < self.memory.len() {
                                let val = self.memory[addr + i] as i64;
                                self.output_queue.push(val);
                                written_bytes += 1;
                            } else { break; }
                        }
                        self.registers.insert(Register::RAX, written_bytes as i64);
                    },
                    60 => { // sys_exit
                        self.finished = true;
                    },
                    _ => { self.error = Some(format!("Unknown syscall: {}", rax)); }
                }
            },
            Instruction::JMP(label) => {
                if let Some(&addr) = self.labels.get(&label) { next_pc = addr; }
                else { self.error = Some(format!("Label not found: {}", label)); }
            },
            Instruction::JZ(label) => {
                if self.zf {
                    if let Some(&addr) = self.labels.get(&label) { next_pc = addr; }
                    else { self.error = Some(format!("Label not found: {}", label)); }
                }
            },
            Instruction::JNZ(label) => {
                if !self.zf {
                    if let Some(&addr) = self.labels.get(&label) { next_pc = addr; }
                    else { self.error = Some(format!("Label not found: {}", label)); }
                }
            },
            Instruction::JS(label) => {
                if self.sf {
                    if let Some(&addr) = self.labels.get(&label) { next_pc = addr; }
                    else { self.error = Some(format!("Label not found: {}", label)); }
                }
            },
            Instruction::RET => { self.finished = true; }
        }

        if self.error.is_none() {
            self.pc = next_pc;
        }

        true
    }
}

pub fn parse_operand(s: &str) -> Result<Operand, String> {
    let s = s.trim();
    if s.starts_with('[') && s.ends_with(']') {
        let content = &s[1..s.len()-1].trim();
        if let Ok(Operand::Reg(r)) = parse_operand(content) {
            return Ok(Operand::MemReg(r));
        } else {
            return Ok(Operand::MemLabel(content.to_string()));
        }
    }

    let clean_s = s.strip_prefix('%').unwrap_or(s);
    match clean_s.to_uppercase().as_str() {
        "RAX" => Ok(Operand::Reg(Register::RAX)),
        "RBX" => Ok(Operand::Reg(Register::RBX)),
        "RCX" => Ok(Operand::Reg(Register::RCX)),
        "RDX" => Ok(Operand::Reg(Register::RDX)),
        "RSI" => Ok(Operand::Reg(Register::RSI)),
        "RDI" => Ok(Operand::Reg(Register::RDI)),
        "RSP" => Ok(Operand::Reg(Register::RSP)),
        "RBP" => Ok(Operand::Reg(Register::RBP)),
        "R8" => Ok(Operand::Reg(Register::R8)),
        "R9" => Ok(Operand::Reg(Register::R9)),
        "R10" => Ok(Operand::Reg(Register::R10)),
        "R11" => Ok(Operand::Reg(Register::R11)),
        "R12" => Ok(Operand::Reg(Register::R12)),
        "R13" => Ok(Operand::Reg(Register::R13)),
        "R14" => Ok(Operand::Reg(Register::R14)),
        "R15" => Ok(Operand::Reg(Register::R15)),
        _ => {
            let imm_s = s.strip_prefix('$').unwrap_or(s);
            if let Ok(val) = imm_s.parse::<i64>() {
                Ok(Operand::Imm(val))
            } else if !s.is_empty() && s.chars().next().map_or(false, |c| !c.is_numeric()) {
                Ok(Operand::Label(s.to_string()))
            } else {
                Err(format!("Unknown operand: {}", s))
            }
        }
    }
}

pub fn parse_program(code: &str, syntax: Syntax) -> Result<(Vec<Instruction>, HashMap<String, usize>, HashMap<String, usize>), String> {
    let mut instructions = Vec::new();
    let mut labels = HashMap::new();
    let mut data_labels = HashMap::new();
    let mut current_bss_offset = 0;
    
    let mut pc_counter = 0;
    let mut current_section = ".text".to_string();

    for line in code.lines() {
        let line = line.split(&[';', '#'][..]).next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }
        
        let mut op_idx = 0;
        let first_part = parts[0];
        
        // Label detection (with or without colon)
        if first_part.ends_with(':') {
            let label_name = first_part[..first_part.len()-1].to_string();
            if current_section == ".text" { labels.insert(label_name, pc_counter); }
            else { data_labels.insert(label_name, current_bss_offset); }
            op_idx = 1;
        } else if (current_section == ".bss" || current_section == "section .bss") && parts.len() > 1 && parts[1].to_lowercase() == "resb" {
            // "buf resb 16"
            let label_name = first_part.to_string();
            data_labels.insert(label_name, current_bss_offset);
            op_idx = 1;
        }

        if op_idx >= parts.len() { continue; }
        let op_raw = parts[op_idx].to_lowercase();
        
        if op_raw == "section" {
            if parts.len() > op_idx + 1 {
                current_section = parts[op_idx + 1].to_string();
            }
            continue;
        }

        if ["global", "extern", "default", "bits"].contains(&op_raw.as_str()) {
            continue;
        }

        if current_section == ".bss" {
            if op_raw == "resb" {
                if parts.len() > op_idx + 1 {
                    if let Ok(size) = parts[op_idx + 1].parse::<usize>() {
                        current_bss_offset += size;
                    }
                }
            }
            continue;
        }

        // TEXT SECTION Parsing
        let args_str = parts[1..].join(" ");
        let args: Vec<&str> = if args_str.is_empty() { Vec::new() } else { args_str.split(',').collect() };

        let inst = match op_raw.as_str() {
            "mov" | "movq" => {
                if args.len() != 2 { return Err(format!("{} requires 2 operands", op_raw)); }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax { Syntax::Intel => Instruction::MOV(op1, op2), Syntax::Att => Instruction::MOV(op2, op1) }
            },
            "add" | "addq" => {
                if args.len() != 2 { return Err(format!("{} requires 2 operands", op_raw)); }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax { Syntax::Intel => Instruction::ADD(op1, op2), Syntax::Att => Instruction::ADD(op2, op1) }
            },
            "sub" | "subq" => {
                if args.len() != 2 { return Err(format!("{} requires 2 operands", op_raw)); }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax { Syntax::Intel => Instruction::SUB(op1, op2), Syntax::Att => Instruction::SUB(op2, op1) }
            },
            "inc" | "incq" => Instruction::INC(parse_operand(args[0])?),
            "dec" | "decq" => Instruction::DEC(parse_operand(args[0])?),
            "xor" | "xorq" => {
                if args.len() != 2 { return Err(format!("{} requires 2 operands", op_raw)); }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax { Syntax::Intel => Instruction::XOR(op1, op2), Syntax::Att => Instruction::XOR(op2, op1) }
            },
            "cmp" | "cmpq" => {
                if args.len() != 2 { return Err(format!("{} requires 2 operands", op_raw)); }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax { Syntax::Intel => Instruction::CMP(op1, op2), Syntax::Att => Instruction::CMP(op2, op1) }
            },
            "test" | "testq" => {
                if args.len() != 2 { return Err(format!("{} requires 2 operands", op_raw)); }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax { Syntax::Intel => Instruction::TEST(op1, op2), Syntax::Att => Instruction::TEST(op2, op1) }
            },
            "push" | "pushq" => Instruction::PUSH(parse_operand(args[0])?),
            "pop" | "popq" => Instruction::POP(parse_operand(args[0])?),
            "in" => Instruction::IN(parse_operand(args[0])?),
            "out" => Instruction::OUT(parse_operand(args[0])?),
            "syscall" => Instruction::SYSCALL,
            "jmp" => Instruction::JMP(parts[1].trim().to_string()),
            "jz" | "je" => Instruction::JZ(parts[1].trim().to_string()),
            "jnz" | "jne" => Instruction::JNZ(parts[1].trim().to_string()),
            "js" => Instruction::JS(parts[1].trim().to_string()),
            "ret" | "retq" => Instruction::RET,
            _ => return Err(format!("Unknown instruction: {}", op_raw))
        };
        instructions.push(inst);
        pc_counter += 1;
    }
    Ok((instructions, labels, data_labels))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let code = "
            mov rax, 10
            add rax, 20
            sub rax, 5
            ; exit(25)
            mov rdi, rax
            mov rax, 60
            syscall
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RDI), 25);
    }

    #[test]
    fn test_memory_and_bss() {
        let code = "
            section .bss
            buf: resb 16
            
            section .text
            global _start
            _start:
            mov rax, 12345
            mov [buf], rax
            mov rbx, [buf]
            ; exit(0)
            mov rax, 60
            syscall
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RBX), 12345);
    }

    #[test]
    fn test_nasm_style_bss() {
        let code = "
            section .bss
            buf resb 16
            
            section .text
            _start:
            mov rax, 99
            mov [buf], rax
            mov rbx, [buf]
            ; exit(0)
            mov rax, 60
            syscall
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RBX), 99);
    }

    #[test]
    fn test_syscall_io() {
        let code = "
            section .bss
            buf resb 8
            
            section .text
            _start:
            ; read 1 byte
            mov rax, 0
            mov rdi, 0
            mov rsi, buf
            mov rdx, 1
            syscall
            
            ; write 1 byte back
            mov rax, 1
            mov rdi, 1
            mov rsi, buf
            mov rdx, 1
            syscall
            
            ; exit(0)
            mov rax, 60
            syscall
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![42]);
        while vm.step() {}
        let state = vm.get_state();
        assert_eq!(state.output, vec![42]);
    }
}
