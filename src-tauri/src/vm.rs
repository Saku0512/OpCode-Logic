use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Register {
    RAX, RBX, RCX, RDX, RSI, RDI, RSP, RBP,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    Reg(Register),
    Imm(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Syntax {
    Intel,
    Att,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    MOV(Operand, Operand), // dest, src
    ADD(Operand, Operand), // dest, src (Intel convention: ADD DEST, SRC)
    SUB(Operand, Operand), // dest, src
    RET,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VmState {
    pub registers: HashMap<Register, i64>,
    pub pc: usize,
    pub finished: bool,
    pub error: Option<String>,
}

pub struct VM {
    registers: HashMap<Register, i64>,
    pc: usize,
    program: Vec<Instruction>,
    error: Option<String>,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> Self {
        let mut registers = HashMap::new();
        // Zero initialize all registers
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
            pc: 0,
            program,
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
            pc: self.pc,
            finished: self.pc >= self.program.len() || self.error.is_some(),
            error: self.error.clone(),
        }
    }

    fn get_value(&self, op: &Operand) -> i64 {
        match op {
            Operand::Reg(r) => *self.registers.get(r).unwrap_or(&0),
            Operand::Imm(val) => *val,
        }
    }

    fn remove_percent(s: &str) -> &str {
        s.strip_prefix('%').unwrap_or(s)
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.program.len() || self.error.is_some() {
            return false;
        }

        let inst = self.program[self.pc].clone();
        
        match inst {
            Instruction::MOV(dest, src) => {
                let val = self.get_value(&src);
                if let Operand::Reg(r) = dest {
                    self.registers.insert(r, val);
                } else {
                    self.error = Some("Invalid destination for MOV".to_string());
                }
            },
            Instruction::ADD(dest, src) => {
                let val = self.get_value(&src);
                if let Operand::Reg(r) = dest {
                    let acc = self.registers.get(&r).unwrap_or(&0);
                    self.registers.insert(r, acc + val);
                } else {
                     self.error = Some("Invalid destination for ADD".to_string());
                }
            },
            Instruction::SUB(dest, src) => {
                let val = self.get_value(&src);
                if let Operand::Reg(r) = dest {
                    let acc = self.registers.get(&r).unwrap_or(&0);
                    self.registers.insert(r, acc - val);
                } else {
                     self.error = Some("Invalid destination for SUB".to_string());
                }
            },
            Instruction::RET => {
                // For now, RET just ends execution by setting PC to end
                self.pc = self.program.len(); // Terminate
                return true;
            }
        };

        if self.error.is_none() && self.pc < self.program.len() {
             self.pc += 1;
        }

        true
    }
}

pub fn parse_operand(s: &str) -> Result<Operand, String> {
    let s = s.trim();
    // Handle % prefix for AT&T parsing if passed raw, 
    // but typically we strip it before calling this or handle it here
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
            // Handle $ prefix for AT&T immediate
             let imm_s = s.strip_prefix('$').unwrap_or(s);
            if let Ok(val) = imm_s.parse::<i64>() {
                Ok(Operand::Imm(val))
            } else {
                Err(format!("Unknown operand: {}", s))
            }
        }
    }
}

pub fn parse_program(code: &str, syntax: Syntax) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();
    for line in code.lines() {
        let line = line.split(&[';', '#'][..]).next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let op_raw = parts[0].to_lowercase(); 
        
        let inst = match op_raw.as_str() {
            "mov" | "movq" => {
                 let args_str = parts[1..].join(" ");
                 let args: Vec<&str> = args_str.split(',').collect();
                 if args.len() != 2 {
                     return Err(format!("{} requires 2 operands", op_raw));
                 }
                 
                 let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                 
                 match syntax {
                     Syntax::Intel => Instruction::MOV(op1, op2), // dest, src
                     Syntax::Att => Instruction::MOV(op2, op1),   // src, dest -> dest, src
                 }
            },
            "add" | "addq" => {
                 let args_str = parts[1..].join(" ");
                 let args: Vec<&str> = args_str.split(',').collect();
                 if args.len() != 2 {
                     return Err(format!("{} requires 2 operands", op_raw));
                 }
                 let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                 match syntax {
                     Syntax::Intel => Instruction::ADD(op1, op2), // dest, src
                     Syntax::Att => Instruction::ADD(op2, op1),   // src, dest -> dest, src
                 }
            },
             "sub" | "subq" => {
                 let args_str = parts[1..].join(" ");
                 let args: Vec<&str> = args_str.split(',').collect();
                 if args.len() != 2 {
                     return Err(format!("{} requires 2 operands", op_raw));
                 }
                 let (op1, op2) = (parse_operand(args[0])?, parse_operand(args[1])?);
                 match syntax {
                     Syntax::Intel => Instruction::SUB(op1, op2), 
                     Syntax::Att => Instruction::SUB(op2, op1),   
                 }
            },
            "ret" | "retq" => Instruction::RET,
            _ => return Err(format!("Unknown instruction: {}", op_raw))
        };
        instructions.push(inst);
    }
    Ok(instructions)
}
