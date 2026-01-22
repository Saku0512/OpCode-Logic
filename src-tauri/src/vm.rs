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
    Label(String), // For Jumps
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
    JMP(String), // Label
    JZ(String),   // Jump if Zero
    JNZ(String),  // Jump if Not Zero
    JS(String),   // Jump if Sign
    PUSH(Operand),
    POP(Operand),
    IN(Operand),  // Pseudo: read next input into operand
    OUT(Operand), // Pseudo: write to output stream
    RET,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VmState {
    pub registers: HashMap<Register, i64>,
    pub zf: bool,
    pub sf: bool,
    pub pc: usize,
    pub output: Vec<i64>, // Stream output
    pub stack: Vec<i64>,  // Stack snapshot
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
    stack: Vec<i64>,
    input_queue: VecDeque<i64>,
    output_queue: Vec<i64>,
    error: Option<String>,
}

impl VM {
    pub fn new(program: Vec<Instruction>, labels: HashMap<String, usize>, input: Vec<i64>) -> Self {
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
            stack: Vec::new(),
            input_queue: VecDeque::from(input),
            output_queue: Vec::new(),
            error: None,
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
            input_remaining: self.input_queue.len(),
            finished: self.pc >= self.program.len() || self.error.is_some(),
            error: self.error.clone(),
        }
    }

    fn get_value(&self, op: &Operand) -> Result<i64, String> {
        match op {
            Operand::Reg(r) => Ok(*self.registers.get(r).unwrap_or(&0)),
            Operand::Imm(val) => Ok(*val),
            Operand::Label(_) => Err("Invalid value operand: Label".to_string()),
        }
    }

    fn update_flags(&mut self, val: i64) {
        self.zf = val == 0;
        self.sf = val < 0;
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.program.len() || self.error.is_some() {
            return false;
        }

        let inst = self.program[self.pc].clone();
        let mut next_pc = self.pc + 1;

        match inst {
            Instruction::MOV(dest, src) => {
                if let Ok(val) = self.get_value(&src) {
                    if let Operand::Reg(r) = dest {
                        self.registers.insert(r, val);
                    } else { self.error = Some("Invalid destination".to_string()); }
                } else { self.error = Some("Invalid source".to_string()); }
            },
            Instruction::ADD(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1.wrapping_add(v2);
                    if let Operand::Reg(r) = dest {
                        self.registers.insert(r, res);
                        self.update_flags(res);
                    } else { self.error = Some("Invalid destination".to_string()); }
                } else { self.error = Some("Invalid operands".to_string()); }
            },
            Instruction::SUB(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1.wrapping_sub(v2);
                    if let Operand::Reg(r) = dest {
                        self.registers.insert(r, res);
                        self.update_flags(res);
                    } else { self.error = Some("Invalid destination".to_string()); }
                } else { self.error = Some("Invalid operands".to_string()); }
            },
            Instruction::INC(op) => {
                if let Ok(val) = self.get_value(&op) {
                    let res = val.wrapping_add(1);
                    if let Operand::Reg(r) = op {
                        self.registers.insert(r, res);
                        self.update_flags(res);
                    } else { self.error = Some("Invalid destination".to_string()); }
                } else { self.error = Some("Invalid operand".to_string()); }
            },
            Instruction::DEC(op) => {
                if let Ok(val) = self.get_value(&op) {
                    let res = val.wrapping_sub(1);
                    if let Operand::Reg(r) = op {
                        self.registers.insert(r, res);
                        self.update_flags(res);
                    } else { self.error = Some("Invalid destination".to_string()); }
                } else { self.error = Some("Invalid operand".to_string()); }
            },
            Instruction::XOR(dest, src) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&dest), self.get_value(&src)) {
                    let res = v1 ^ v2;
                    if let Operand::Reg(r) = dest {
                        self.registers.insert(r, res);
                        self.update_flags(res);
                    } else { self.error = Some("Invalid destination".to_string()); }
                } else { self.error = Some("Invalid operands".to_string()); }
            },
            Instruction::CMP(op1, op2) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&op1), self.get_value(&op2)) {
                    let res = v1.wrapping_sub(v2);
                    self.update_flags(res);
                } else { self.error = Some("Invalid operands".to_string()); }
            },
            Instruction::TEST(op1, op2) => {
                if let (Ok(v1), Ok(v2)) = (self.get_value(&op1), self.get_value(&op2)) {
                    let res = v1 & v2;
                    self.update_flags(res);
                } else { self.error = Some("Invalid operands".to_string()); }
            },
            Instruction::PUSH(op) => {
                if let Ok(val) = self.get_value(&op) {
                    self.stack.push(val);
                } else { self.error = Some("Invalid source".to_string()); }
            },
            Instruction::POP(op) => {
                if let Some(val) = self.stack.pop() {
                    if let Operand::Reg(r) = op {
                        self.registers.insert(r, val);
                    } else { self.error = Some("Invalid destination".to_string()); }
                } else { self.error = Some("Stack underflow".to_string()); }
            },
            Instruction::IN(op) => {
                if let Some(val) = self.input_queue.pop_front() {
                    if let Operand::Reg(r) = op {
                        self.registers.insert(r, val);
                    } else { self.error = Some("Invalid destination for IN".to_string()); }
                } else {
                    self.error = Some("Input buffer empty".to_string());
                }
            },
            Instruction::OUT(op) => {
                if let Ok(val) = self.get_value(&op) {
                    self.output_queue.push(val);
                } else { self.error = Some("Invalid source for OUT".to_string()); }
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
            Instruction::RET => { next_pc = self.program.len(); }
        }

        if self.error.is_none() {
            self.pc = next_pc;
        }

        true
    }
}

pub fn parse_operand(s: &str) -> Result<Operand, String> {
    let s = s.trim();
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

pub fn parse_program(code: &str, syntax: Syntax) -> Result<(Vec<Instruction>, HashMap<String, usize>), String> {
    let mut instructions = Vec::new();
    let mut labels = HashMap::new();
    
    let mut pc_counter = 0;
    for line in code.lines() {
        let line = line.split(&[';', '#'][..]).next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        
        let mut clean_line = line.to_string();
        if let Some(idx) = line.find(':') {
            let label_name = line[..idx].trim().to_string();
            labels.insert(label_name, pc_counter);
            if line.len() > idx + 1 {
                clean_line = line[idx+1..].trim().to_string();
                if clean_line.is_empty() { continue; }
            } else { continue; }
        }

        let parts: Vec<&str> = clean_line.split_whitespace().collect();
        let op_raw = parts[0].to_lowercase();
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
            "jmp" => Instruction::JMP(parts[1].to_string()),
            "jz" | "je" => Instruction::JZ(parts[1].to_string()),
            "jnz" | "jne" => Instruction::JNZ(parts[1].to_string()),
            "js" => Instruction::JS(parts[1].to_string()),
            "ret" | "retq" => Instruction::RET,
            _ => return Err(format!("Unknown instruction: {}", op_raw))
        };
        instructions.push(inst);
        pc_counter += 1;
    }
    Ok((instructions, labels))
}
