use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    Reg(Register),
    Imm(i64),
    Label(String),
    MemReg(Register), // [RAX]
    MemLabel(String), // [buf]
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
    pub exited: bool, // sys_exit で終了したかどうか
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
    exited: bool,               // sys_exit で終了したかどうか
    execution_log: Vec<String>, // 実行ログを保存
}

const MEMORY_SIZE: usize = 65536; // 64KB

impl VM {
    pub fn new(
        program: Vec<Instruction>,
        labels: HashMap<String, usize>,
        data_labels: HashMap<String, usize>,
        input: Vec<i64>,
    ) -> Self {
        let mut registers = HashMap::new();
        for r in [
            Register::RAX,
            Register::RBX,
            Register::RCX,
            Register::RDX,
            Register::RSI,
            Register::RDI,
            Register::RSP,
            Register::RBP,
            Register::R8,
            Register::R9,
            Register::R10,
            Register::R11,
            Register::R12,
            Register::R13,
            Register::R14,
            Register::R15,
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
            exited: false,
            execution_log: Vec::new(),
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
            exited: self.exited,
            error: self.error.clone(),
        }
    }

    pub fn get_execution_log(&self) -> Vec<String> {
        self.execution_log.clone()
    }

    fn log(&mut self, message: String) {
        // コンソールにも出力（デバッグ用）
        println!("{}", message);
        self.execution_log.push(message);
    }

    fn format_operand(&self, op: &Operand) -> String {
        match op {
            Operand::Reg(r) => format!("{:?}", r),
            Operand::Imm(val) => format!("{}", val),
            Operand::Label(l) => l.clone(),
            Operand::MemReg(r) => format!("[{:?}]", r),
            Operand::MemLabel(l) => format!("[{}]", l),
        }
    }

    fn format_instruction(&self, inst: &Instruction) -> String {
        match inst {
            Instruction::MOV(dest, src) => format!(
                "MOV {}, {}",
                self.format_operand(dest),
                self.format_operand(src)
            ),
            Instruction::ADD(dest, src) => format!(
                "ADD {}, {}",
                self.format_operand(dest),
                self.format_operand(src)
            ),
            Instruction::SUB(dest, src) => format!(
                "SUB {}, {}",
                self.format_operand(dest),
                self.format_operand(src)
            ),
            Instruction::INC(op) => format!("INC {}", self.format_operand(op)),
            Instruction::DEC(op) => format!("DEC {}", self.format_operand(op)),
            Instruction::XOR(dest, src) => format!(
                "XOR {}, {}",
                self.format_operand(dest),
                self.format_operand(src)
            ),
            Instruction::CMP(op1, op2) => format!(
                "CMP {}, {}",
                self.format_operand(op1),
                self.format_operand(op2)
            ),
            Instruction::TEST(op1, op2) => format!(
                "TEST {}, {}",
                self.format_operand(op1),
                self.format_operand(op2)
            ),
            Instruction::JMP(label) => format!("JMP {}", label),
            Instruction::JZ(label) => format!("JZ {}", label),
            Instruction::JNZ(label) => format!("JNZ {}", label),
            Instruction::JS(label) => format!("JS {}", label),
            Instruction::PUSH(op) => format!("PUSH {}", self.format_operand(op)),
            Instruction::POP(op) => format!("POP {}", self.format_operand(op)),
            Instruction::IN(op) => format!("IN {}", self.format_operand(op)),
            Instruction::OUT(op) => format!("OUT {}", self.format_operand(op)),
            Instruction::SYSCALL => "SYSCALL".to_string(),
            Instruction::RET => "RET".to_string(),
        }
    }

    fn log_registers(&self) -> String {
        let regs = vec![
            ("RAX", Register::RAX),
            ("RBX", Register::RBX),
            ("RCX", Register::RCX),
            ("RDX", Register::RDX),
            ("RSI", Register::RSI),
            ("RDI", Register::RDI),
        ];
        let mut result = String::new();
        for (name, reg) in regs {
            let val = self.get_register(reg);
            if val != 0 || name == "RDI" || name == "RSI" || name == "RDX" {
                result.push_str(&format!("{}={} ", name, val));
            }
        }
        result.trim().to_string()
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
            }
            Operand::MemReg(r) => {
                let addr = self.get_register(*r) as usize;
                if addr + 8 <= self.memory.len() {
                    let bytes = &self.memory[addr..addr + 8];
                    Ok(i64::from_le_bytes(bytes.try_into().unwrap()))
                } else {
                    Err(format!("Segmentation fault at address 0x{:x}", addr))
                }
            }
            Operand::MemLabel(l) => {
                if let Some(&addr) = self.data_labels.get(l) {
                    if addr + 8 <= self.memory.len() {
                        let bytes = &self.memory[addr..addr + 8];
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
            }
            Operand::MemReg(r) => {
                let addr = self.get_register(*r) as usize;
                if addr < self.memory.len() {
                    // Supporting partial writes is complex, but for now let's do byte if it's a small value??
                    // Real x86_64 mov [buf], rax writes 8 bytes.
                    if addr + 8 <= self.memory.len() {
                        let bytes = val.to_le_bytes();
                        self.memory[addr..addr + 8].copy_from_slice(&bytes);
                        Ok(())
                    } else {
                        self.memory[addr] = (val & 0xFF) as u8;
                        Ok(())
                    }
                } else {
                    Err(format!("Segmentation fault (store) at 0x{:x}", addr))
                }
            }
            Operand::MemLabel(l) => {
                if let Some(&addr) = self.data_labels.get(l) {
                    if addr + 8 <= self.memory.len() {
                        let bytes = val.to_le_bytes();
                        self.memory[addr..addr + 8].copy_from_slice(&bytes);
                        Ok(())
                    } else {
                        self.memory[addr] = (val & 0xFF) as u8;
                        Ok(())
                    }
                } else {
                    Err(format!("Symbol not found: {}", l))
                }
            }
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

        // 実行前の状態をログ
        let inst_str = self.format_instruction(&inst);
        let regs_before = self.log_registers();
        self.log(format!(
            "[Step {}] PC={} | {} | Before: {} | ZF={} SF={}",
            self.execution_log.len() + 1,
            self.pc,
            inst_str,
            regs_before,
            self.zf,
            self.sf
        ));

        match inst {
            Instruction::MOV(dest, src) => match self.get_value(&src) {
                Ok(val) => {
                    if let Err(e) = self.set_value(&dest, val) {
                        self.error = Some(e.clone());
                        self.log(format!("  ERROR: {}", e));
                    } else {
                        let dest_str = self.format_operand(&dest);
                        self.log(format!("  -> {} = {}", dest_str, val));
                    }
                }
                Err(e) => {
                    self.error = Some(e.clone());
                    self.log(format!("  ERROR: {}", e));
                }
            },
            Instruction::ADD(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1.wrapping_add(v2);
                    let _ = self.set_value(&dest, res);
                    self.update_flags(res);
                    let dest_str = self.format_operand(&dest);
                    self.log(format!("  -> {} = {} + {} = {}", dest_str, v1, v2, res));
                } else {
                    self.error = Some("Invalid operands for ADD".to_string());
                    self.log("  ERROR: Invalid operands for ADD".to_string());
                }
            }
            Instruction::SUB(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1.wrapping_sub(v2);
                    let _ = self.set_value(&dest, res);
                    self.update_flags(res);
                    let dest_str = self.format_operand(&dest);
                    self.log(format!("  -> {} = {} - {} = {}", dest_str, v1, v2, res));
                } else {
                    self.error = Some("Invalid operands for SUB".to_string());
                    self.log("  ERROR: Invalid operands for SUB".to_string());
                }
            }
            Instruction::INC(op) => {
                if let Ok(val) = self.get_value(&op) {
                    let res = val.wrapping_add(1);
                    let _ = self.set_value(&op, res);
                    self.update_flags(res);
                } else {
                    self.error = Some("Invalid operand for INC".to_string());
                }
            }
            Instruction::DEC(op) => {
                if let Ok(val) = self.get_value(&op) {
                    let res = val.wrapping_sub(1);
                    let _ = self.set_value(&op, res);
                    self.update_flags(res);
                } else {
                    self.error = Some("Invalid operand for DEC".to_string());
                }
            }
            Instruction::XOR(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1 ^ v2;
                    let _ = self.set_value(&dest, res);
                    self.update_flags(res);
                } else {
                    self.error = Some("Invalid operands for XOR".to_string());
                }
            }
            Instruction::CMP(op1, op2) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&op1), self.get_value(&op2)) {
                    let res = v1.wrapping_sub(v2);
                    self.update_flags(res);
                } else {
                    self.error = Some("Invalid operands for CMP".to_string());
                }
            }
            Instruction::TEST(op1, op2) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&op1), self.get_value(&op2)) {
                    let res = v1 & v2;
                    self.update_flags(res);
                } else {
                    self.error = Some("Invalid operands for TEST".to_string());
                }
            }
            Instruction::PUSH(op) => {
                if let Ok(val) = self.get_value(&op) {
                    self.stack.push(val);
                } else {
                    self.error = Some("Invalid source for PUSH".to_string());
                }
            }
            Instruction::POP(op) => {
                if let Some(val) = self.stack.pop() {
                    if let Err(e) = self.set_value(&op, val) {
                        self.error = Some(e);
                    }
                } else {
                    self.error = Some("Stack underflow".to_string());
                }
            }
            Instruction::IN(op) => {
                if let Some(val) = self.input_queue.pop_front() {
                    if let Err(e) = self.set_value(&op, val) {
                        self.error = Some(e.clone());
                        self.log(format!("  ERROR: {}", e));
                    } else {
                        let op_str = self.format_operand(&op);
                        self.log(format!(
                            "  -> {} = INPUT: {} (remaining: {})",
                            op_str,
                            val,
                            self.input_queue.len()
                        ));
                    }
                } else {
                    self.error = Some("Input buffer empty".to_string());
                    self.log("  ERROR: Input buffer empty".to_string());
                }
            }
            Instruction::OUT(op) => {
                if let Ok(val) = self.get_value(&op) {
                    self.output_queue.push(val);
                    self.log(format!(
                        "  -> OUTPUT: {} (total outputs: {})",
                        val,
                        self.output_queue.len()
                    ));
                } else {
                    self.error = Some("Invalid source for OUT".to_string());
                    self.log("  ERROR: Invalid source for OUT".to_string());
                }
            }
            Instruction::SYSCALL => {
                let rax = self.get_register(Register::RAX);
                match rax {
                    0 => {
                        // sys_read
                        let count = self.get_register(Register::RDX) as usize;
                        let addr = self.get_register(Register::RSI) as usize;
                        let mut read_bytes = 0;
                        self.log(format!(
                            "  -> SYSCALL READ: count={}, addr=0x{:x}",
                            count, addr
                        ));
                        for i in 0..count {
                            if let Some(val) = self.input_queue.pop_front() {
                                if addr + i < self.memory.len() {
                                    let byte_val = (val & 0xFF) as u8;
                                    self.memory[addr + i] = byte_val;
                                    self.log(format!("    Read byte[{}]: input={} -> memory[0x{:x}]={} (0x{:02x})", 
                                        i, val, addr + i, byte_val as i8 as i64, byte_val));
                                    read_bytes += 1;
                                }
                            } else {
                                break;
                            }
                        }
                        self.registers.insert(Register::RAX, read_bytes as i64);
                        self.log(format!("  -> READ complete: {} bytes read", read_bytes));
                    }
                    1 => {
                        // sys_write
                        let count = self.get_register(Register::RDX) as usize;
                        let addr = self.get_register(Register::RSI) as usize;
                        let mut written_bytes = 0;
                        self.log(format!(
                            "  -> SYSCALL WRITE: count={}, addr=0x{:x}",
                            count, addr
                        ));
                        for i in 0..count {
                            if addr + i < self.memory.len() {
                                // 符号拡張: u8 を i64 に変換する際、最上位ビットが1なら符号拡張
                                let byte_val = self.memory[addr + i];
                                let val = if byte_val & 0x80 != 0 {
                                    // 符号拡張: 負の値として扱う
                                    (byte_val as i8) as i64
                                } else {
                                    byte_val as i64
                                };
                                self.log(format!(
                                    "    Write byte[{}]: memory[0x{:x}]=0x{:02x} -> output={}",
                                    i,
                                    addr + i,
                                    byte_val,
                                    val
                                ));
                                self.output_queue.push(val);
                                written_bytes += 1;
                            } else {
                                break;
                            }
                        }
                        self.registers.insert(Register::RAX, written_bytes as i64);
                        self.log(format!(
                            "  -> WRITE complete: {} bytes written, output queue: {:?}",
                            written_bytes, self.output_queue
                        ));
                    }
                    60 => {
                        // sys_exit
                        self.finished = true;
                        self.exited = true;
                    }
                    _ => {
                        self.error = Some(format!("Unknown syscall: {}", rax));
                    }
                }
            }
            Instruction::JMP(label) => {
                if let Some(&addr) = self.labels.get(&label) {
                    next_pc = addr;
                    self.log(format!(
                        "  -> JUMP to {} (PC: {} -> {})",
                        label, self.pc, addr
                    ));
                } else {
                    self.error = Some(format!("Label not found: {}", label));
                    self.log(format!("  ERROR: Label not found: {}", label));
                }
            }
            Instruction::JZ(label) => {
                if self.zf {
                    if let Some(&addr) = self.labels.get(&label) {
                        next_pc = addr;
                        self.log(format!(
                            "  -> JZ taken: jump to {} (PC: {} -> {})",
                            label, self.pc, addr
                        ));
                    } else {
                        self.error = Some(format!("Label not found: {}", label));
                        self.log(format!("  ERROR: Label not found: {}", label));
                    }
                } else {
                    self.log(format!("  -> JZ not taken (ZF={})", self.zf));
                }
            }
            Instruction::JNZ(label) => {
                if !self.zf {
                    if let Some(&addr) = self.labels.get(&label) {
                        next_pc = addr;
                        self.log(format!(
                            "  -> JNZ taken: jump to {} (PC: {} -> {})",
                            label, self.pc, addr
                        ));
                    } else {
                        self.error = Some(format!("Label not found: {}", label));
                        self.log(format!("  ERROR: Label not found: {}", label));
                    }
                } else {
                    self.log(format!("  -> JNZ not taken (ZF={})", self.zf));
                }
            }
            Instruction::JS(label) => {
                if self.sf {
                    if let Some(&addr) = self.labels.get(&label) {
                        next_pc = addr;
                        self.log(format!(
                            "  -> JS taken: jump to {} (PC: {} -> {})",
                            label, self.pc, addr
                        ));
                    } else {
                        self.error = Some(format!("Label not found: {}", label));
                        self.log(format!("  ERROR: Label not found: {}", label));
                    }
                } else {
                    self.log(format!("  -> JS not taken (SF={})", self.sf));
                }
            }
            Instruction::RET => {
                self.finished = true;
            }
        }

        if self.error.is_none() {
            self.pc = next_pc;
            let regs_after = self.log_registers();
            self.log(format!(
                "  After: {} | ZF={} SF={} | Output: {:?}",
                regs_after, self.zf, self.sf, self.output_queue
            ));
        } else {
            self.log("  EXECUTION STOPPED due to error".to_string());
        }

        true
    }
}

pub fn parse_operand(s: &str) -> Result<Operand, String> {
    let s = s.trim();
    if s.starts_with('[') && s.ends_with(']') {
        let content = &s[1..s.len() - 1].trim();
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
            } else if !s.is_empty() && s.chars().next().is_some_and(|c| !c.is_numeric()) {
                Ok(Operand::Label(s.to_string()))
            } else {
                Err(format!("Unknown operand: {}", s))
            }
        }
    }
}

type ParseResult = Result<
    (
        Vec<Instruction>,
        HashMap<String, usize>,
        HashMap<String, usize>,
    ),
    String,
>;

pub fn parse_program(code: &str, syntax: Syntax) -> ParseResult {
    let mut instructions = Vec::new();
    let mut labels = HashMap::new();
    let mut data_labels = HashMap::new();
    let mut current_bss_offset = 0;

    let mut pc_counter = 0;
    let mut current_section = ".text".to_string();

    for line in code.lines() {
        let line = line.split(&[';', '#'][..]).next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let mut op_idx = 0;
        let first_part = parts[0];

        // Label detection (with or without colon)
        if let Some(stripped) = first_part.strip_suffix(':') {
            let label_name = stripped.to_string();
            if current_section == ".text" {
                labels.insert(label_name, pc_counter);
            } else {
                data_labels.insert(label_name, current_bss_offset);
            }
            op_idx = 1;
        } else if (current_section == ".bss" || current_section == "section .bss")
            && parts.len() > 1
            && parts[1].to_lowercase() == "resb"
        {
            // "buf resb 16"
            let label_name = first_part.to_string();
            data_labels.insert(label_name, current_bss_offset);
            op_idx = 1;
        }

        if op_idx >= parts.len() {
            continue;
        }
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
            if op_raw == "resb" && parts.len() > op_idx + 1 {
                if let Ok(size) = parts[op_idx + 1].parse::<usize>() {
                    current_bss_offset += size;
                }
            }
            continue;
        }

        // TEXT SECTION Parsing
        let args_str = parts[1..].join(" ");
        let args: Vec<&str> = if args_str.is_empty() {
            Vec::new()
        } else {
            args_str.split(',').collect()
        };

        let inst = match op_raw.as_str() {
            "mov" | "movq" => {
                if args.len() != 2 {
                    return Err(format!("{} requires 2 operands", op_raw));
                }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax {
                    Syntax::Intel => Instruction::MOV(op1, op2),
                    Syntax::Att => Instruction::MOV(op2, op1),
                }
            }
            "add" | "addq" => {
                if args.len() != 2 {
                    return Err(format!("{} requires 2 operands", op_raw));
                }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax {
                    Syntax::Intel => Instruction::ADD(op1, op2),
                    Syntax::Att => Instruction::ADD(op2, op1),
                }
            }
            "sub" | "subq" => {
                if args.len() != 2 {
                    return Err(format!("{} requires 2 operands", op_raw));
                }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax {
                    Syntax::Intel => Instruction::SUB(op1, op2),
                    Syntax::Att => Instruction::SUB(op2, op1),
                }
            }
            "inc" | "incq" => Instruction::INC(parse_operand(args[0])?),
            "dec" | "decq" => Instruction::DEC(parse_operand(args[0])?),
            "xor" | "xorq" => {
                if args.len() != 2 {
                    return Err(format!("{} requires 2 operands", op_raw));
                }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax {
                    Syntax::Intel => Instruction::XOR(op1, op2),
                    Syntax::Att => Instruction::XOR(op2, op1),
                }
            }
            "cmp" | "cmpq" => {
                if args.len() != 2 {
                    return Err(format!("{} requires 2 operands", op_raw));
                }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax {
                    Syntax::Intel => Instruction::CMP(op1, op2),
                    Syntax::Att => Instruction::CMP(op2, op1),
                }
            }
            "test" | "testq" => {
                if args.len() != 2 {
                    return Err(format!("{} requires 2 operands", op_raw));
                }
                let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                match syntax {
                    Syntax::Intel => Instruction::TEST(op1, op2),
                    Syntax::Att => Instruction::TEST(op2, op1),
                }
            }
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
            _ => return Err(format!("Unknown instruction: {}", op_raw)),
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

    #[test]
    fn test_mov_instructions() {
        let code = "
            mov rax, 100
            mov rbx, rax
            mov rcx, -50
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 100);
        assert_eq!(vm.get_register(Register::RBX), 100);
        assert_eq!(vm.get_register(Register::RCX), -50);
    }

    #[test]
    fn test_add_instructions() {
        let code = "
            mov rax, 10
            add rax, 20
            add rax, -5
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 25);
    }

    #[test]
    fn test_sub_instructions() {
        let code = "
            mov rax, 50
            sub rax, 20
            sub rax, 10
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 20);
    }

    #[test]
    fn test_inc_dec_instructions() {
        let code = "
            mov rax, 10
            inc rax
            inc rax
            dec rax
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 11);
    }

    #[test]
    fn test_xor_instruction() {
        let code = "
            mov rax, 10
            xor rax, rax
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 0);
        let state = vm.get_state();
        assert!(state.zf);
    }

    #[test]
    fn test_zero_flag() {
        let code = "
            mov rax, 0
            add rax, 0
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        let state = vm.get_state();
        assert!(state.zf);
        assert!(!state.sf);
    }

    #[test]
    fn test_sign_flag() {
        let code = "
            mov rax, -10
            add rax, 0
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        let state = vm.get_state();
        assert!(!state.zf);
        assert!(state.sf);
    }

    #[test]
    fn test_cmp_instruction() {
        let code = "
            mov rax, 10
            mov rbx, 20
            cmp rax, rbx
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        let state = vm.get_state();
        assert!(!state.zf);
        assert!(state.sf); // 10 - 20 = -10, negative
    }

    #[test]
    fn test_jmp_instruction() {
        let code = "
            mov rax, 1
            jmp skip
            mov rax, 2
        skip:
            mov rax, 3
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 3);
    }

    #[test]
    fn test_jz_instruction() {
        let code = "
            mov rax, 0
            add rax, 0
            jz zero_label
            mov rax, 99
            jmp end
        zero_label:
            mov rax, 42
        end:
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 42);
    }

    #[test]
    fn test_jnz_instruction() {
        let code = "
            mov rax, 5
            add rax, 0
            jnz non_zero_label
            mov rax, 99
            jmp end
        non_zero_label:
            mov rax, 42
        end:
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 42);
    }

    #[test]
    fn test_js_instruction() {
        let code = "
            mov rax, -5
            add rax, 0
            js negative_label
            mov rax, 99
            jmp end
        negative_label:
            mov rax, 42
        end:
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 42);
    }

    #[test]
    fn test_push_pop() {
        let code = "
            mov rax, 100
            push rax
            mov rax, 0
            pop rbx
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RBX), 100);
    }

    #[test]
    fn test_in_out_instructions() {
        let code = "
            in rax
            out rax
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![123]);
        while vm.step() {}
        let state = vm.get_state();
        assert_eq!(state.output, vec![123]);
    }

    #[test]
    fn test_ret_instruction() {
        let code = "
            mov rax, 42
            ret
            mov rax, 99
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 42);
        let state = vm.get_state();
        assert!(state.finished);
    }

    #[test]
    fn test_memory_operations() {
        let code = "
            section .bss
            buf resb 16
            
            section .text
            mov rax, 12345
            mov [buf], rax
            mov rbx, [buf]
            mov rcx, rbx
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RBX), 12345);
        assert_eq!(vm.get_register(Register::RCX), 12345);
    }

    #[test]
    fn test_att_syntax() {
        let code = "
            movq $100, %rax
            movq %rax, %rbx
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Att).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 100);
        assert_eq!(vm.get_register(Register::RBX), 100);
    }

    #[test]
    fn test_parse_operand_register() {
        assert!(matches!(
            parse_operand("rax").unwrap(),
            Operand::Reg(Register::RAX)
        ));
        assert!(matches!(
            parse_operand("RAX").unwrap(),
            Operand::Reg(Register::RAX)
        ));
        assert!(matches!(
            parse_operand("%rax").unwrap(),
            Operand::Reg(Register::RAX)
        ));
    }

    #[test]
    fn test_parse_operand_immediate() {
        assert!(matches!(parse_operand("100").unwrap(), Operand::Imm(100)));
        assert!(matches!(parse_operand("$100").unwrap(), Operand::Imm(100)));
        assert!(matches!(parse_operand("-50").unwrap(), Operand::Imm(-50)));
    }

    #[test]
    fn test_parse_operand_memory() {
        assert!(matches!(
            parse_operand("[rax]").unwrap(),
            Operand::MemReg(Register::RAX)
        ));
        assert!(matches!(
            parse_operand("[buf]").unwrap(),
            Operand::MemLabel(_)
        ));
    }

    #[test]
    fn test_wrapping_arithmetic() {
        let code = "
            mov rax, 9223372036854775807
            add rax, 1
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        // Should wrap around to negative
        assert_eq!(vm.get_register(Register::RAX), -9223372036854775808i64);
    }

    #[test]
    fn test_multiple_registers() {
        let code = "
            mov rax, 1
            mov rbx, 2
            mov rcx, 3
            mov rdx, 4
            mov r8, 8
            mov r9, 9
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 1);
        assert_eq!(vm.get_register(Register::RBX), 2);
        assert_eq!(vm.get_register(Register::RCX), 3);
        assert_eq!(vm.get_register(Register::RDX), 4);
        assert_eq!(vm.get_register(Register::R8), 8);
        assert_eq!(vm.get_register(Register::R9), 9);
    }

    #[test]
    fn test_loop_structure() {
        let code = "
            mov rax, 0
            mov rcx, 5
        loop_start:
            add rax, 1
            dec rcx
            jnz loop_start
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        let mut steps = 0;
        while vm.step() && steps < 100 {
            steps += 1;
        }
        assert_eq!(vm.get_register(Register::RAX), 5);
        assert_eq!(vm.get_register(Register::RCX), 0);
    }

    #[test]
    fn test_parse_empty_program() {
        let (prog, labels, data) = parse_program("", Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 0);
        assert_eq!(labels.len(), 0);
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_parse_comments() {
        let code = "
            ; This is a comment
            mov rax, 10 ; Inline comment
            # Another comment style
            mov rbx, 20
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 2);
    }

    #[test]
    fn test_parse_multiple_labels() {
        let code = "
        label1:
            mov rax, 1
        label2:
            mov rbx, 2
        label3:
            mov rcx, 3
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert!(labels.contains_key("label1"));
        assert!(labels.contains_key("label2"));
        assert!(labels.contains_key("label3"));
    }

    #[test]
    fn test_parse_whitespace_handling() {
        let code = "
            mov   rax,   10
            mov    rbx,20
            mov rcx, 30
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 3);
    }

    #[test]
    fn test_parse_directive_ignoring() {
        let code = "
            global _start
            extern printf
            default rel
            bits 64
            mov rax, 10
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 1);
    }

    #[test]
    fn test_parse_invalid_instruction() {
        let result = parse_program("invalid_instruction rax, rbx", Syntax::Intel);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_operands() {
        let result = parse_program("mov rax", Syntax::Intel);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_all_registers() {
        let code = "
            mov rax, 1
            mov rbx, 2
            mov rcx, 3
            mov rdx, 4
            mov rsi, 5
            mov rdi, 6
            mov rsp, 7
            mov rbp, 8
            mov r8, 9
            mov r9, 10
            mov r10, 11
            mov r11, 12
            mov r12, 13
            mov r13, 14
            mov r14, 15
            mov r15, 16
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 16);
    }

    #[test]
    fn test_parse_instruction_variants() {
        let code = "
            mov rax, 1
            movq rax, 2
            add rax, 3
            addq rax, 4
            sub rax, 5
            subq rax, 6
            inc rax
            incq rax
            dec rax
            decq rax
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 10);
    }

    #[test]
    fn test_parse_jump_variants() {
        let code = "
        label:
            jmp label
            jz label
            je label
            jnz label
            jne label
            js label
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 6);
    }

    #[test]
    fn test_parse_ret_variants() {
        let code = "
            ret
            retq
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 2);
    }

    #[test]
    fn test_parse_push_pop_variants() {
        let code = "
            push rax
            pushq rax
            pop rbx
            popq rbx
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert_eq!(prog.len(), 4);
    }

    #[test]
    fn test_parse_large_immediate() {
        let code = "
            mov rax, 9223372036854775807
            mov rbx, -9223372036854775808
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        let mut vm = VM::new(prog, labels, data, vec![]);
        while vm.step() {}
        assert_eq!(vm.get_register(Register::RAX), 9223372036854775807i64);
        assert_eq!(vm.get_register(Register::RBX), -9223372036854775808i64);
    }

    #[test]
    fn test_parse_memory_operations() {
        let code = "
            section .bss
            buf1 resb 8
            buf2 resb 16
            
            section .text
            mov rax, 100
            mov [buf1], rax
            mov rbx, [buf1]
            mov [rbx], rax
            mov rcx, [rbx]
        ";
        let (prog, labels, data) = parse_program(code, Syntax::Intel).unwrap();
        assert!(data.contains_key("buf1"));
        assert!(data.contains_key("buf2"));
        assert_eq!(prog.len(), 5);
    }
}
