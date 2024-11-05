use std::fmt;

use crate::vm::insn::{Insn, Opcode};

const INSN_SIZE: usize = 128; // We can load 128 instructions
const DATA_SIZE: usize = 1024;
const REGS: usize = 256; // We have 256 registers

// Instructions are 64 bits (8 bytes)
// Data is 32 bits (4 bytes)
// We separate instruction and data.
pub struct Cpu {
    insn: [u64; INSN_SIZE], // instruction are 64 bits long
    data: [u32; DATA_SIZE], // data are words of 32 bits
    ip: usize,              // Instruction pointer
    regs: [u32; 256],       // We have 255 registers
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nb = 5;

        writeln!(f, "ip: {}", self.ip)?;

        writeln!(f, "first instructions:")?;
        for idx in 0..nb {
            writeln!(f, "  insn[{}]: {}", idx, self.insn[idx])?;
        }

        writeln!(f, "first data:")?;
        for idx in 0..nb {
            writeln!(f, "  data[{}]: {}", idx, self.data[idx])?;
        }

        writeln!(f, "first registers:")?;
        for idx in 0..nb {
            writeln!(f, "  regs[{}]: {}", idx, self.regs[idx])?;
        }

        Ok(())
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        // We start by executing the instruction at address 0
        Cpu {
            insn: [0; INSN_SIZE],
            data: [0; DATA_SIZE],
            ip: 0,
            regs: [0; REGS],
        }
    }

    pub fn load(&mut self, program: &str) {
        // The program is loaded at offset 0 of insn
        let mut idx: usize = 0;
        for s in program.split("\n") {
            if s.is_empty() {
                continue;
            }
            let instruction = Insn::bin_translation(s);
            self.insn[idx] = instruction.to_u64();
            idx += 1;
        }
    }

    fn step(&mut self) -> bool {
        let insn = Insn::new(self.insn[self.ip]);
        let opcode = match insn.get_opcode() {
            Some(op) => op,
            None => return false,
        };
        match opcode {
            Opcode::Load => {
                let mem = insn.get_immediate() as usize;
                let dest = insn.get_dest() as usize;
                self.regs[dest] = self.data[mem];
            }
            Opcode::Store => {
                let mem = insn.get_immediate() as usize;
                let src1 = insn.get_src1() as usize;
                self.data[mem] = self.regs[src1];
            }
            Opcode::Move => {
                let imm = insn.get_immediate();
                let reg = insn.get_dest() as usize;
                self.regs[reg] = imm;
            }
            Opcode::Add => {
                let src1 = insn.get_src1() as usize;
                let src2 = insn.get_src2() as usize;
                let dest = insn.get_dest() as usize;
                self.regs[dest] = self.regs[src1] + self.regs[src2];
            }
            Opcode::Sub => {
                let src1 = insn.get_src1() as usize;
                let src2 = insn.get_src2() as usize;
                let dest = insn.get_dest() as usize;
                self.regs[dest] = self.regs[src1] - self.regs[src2];
            }
        }
        self.ip += 1;
        true
    }

    pub fn run(&mut self, debug: bool) {
        loop {
            if !self.step() {
                println!("No more steps...");
                break;
            }

            if debug {
                println!("CPU state:\n{}", self);
            }
        }
    }
}
