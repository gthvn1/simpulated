use std::fmt;

use crate::vm::insn;

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
        // Print the first ten instructions
        writeln!(f, "ip: {}", self.ip)?;
        writeln!(f, "first instructions:")?;
        let nb = 5;
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
            self.insn[idx] = insn::bin_translation(s);
            idx += 1;
        }
    }

    pub fn run(&self, debug: bool) {
        let _ = debug;
        todo!("run the program");
    }
}
