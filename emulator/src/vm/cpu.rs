use prettytable::{format, Cell, Row, Table};
use std::{
    fmt,
    io::{self, Write},
};

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

    pub fn run(&mut self) {
        // Display the initial step
        self.display_cpu_state();

        loop {
            if !self.step() {
                println!("Emulation done...");
                break;
            }

            // Sleep 3 seconds to show CPU state
            std::thread::sleep(std::time::Duration::from_secs(3));
            self.display_cpu_state();
            println!("Next step in 3 seconds...");
        }
    }

    fn display_cpu_state(&self) {
        // Clear the terminal screen
        print!("\x1B[2J\x1B[1;1H"); // ANSI escape code to clear screen
        io::stdout().flush().unwrap();

        // ===============================================================================
        // INSTRUCTION TABLE
        let mut insn_table = Table::new();

        // Set table format to remove borders from title row
        insn_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // Colored title (ANSI escape code for blue text)
        let insn_title = "\x1b[34m\x1b[48;5;15mFirst Instructions\x1b[0m"; // Blue color
        let mut insn_title_cell = Cell::new(insn_title).with_hspan(2);
        insn_title_cell.align(prettytable::format::Alignment::CENTER);
        insn_table.add_row(Row::new(vec![insn_title_cell]));

        insn_table.add_row(Row::new(vec![Cell::new("Index"), Cell::new("Instruction")]));
        for idx in 0..5 {
            let instruction = if idx == self.ip {
                format!("\x1b[31m0x{:016x}\x1b[0m", self.insn[idx]) // Red color
            } else {
                format!("0x{:016x}", self.insn[idx])
            };
            insn_table.add_row(Row::new(vec![
                Cell::new(&format!("{}", idx)),
                Cell::new(&instruction),
            ]));
        }
        insn_table.printstd();

        // ===============================================================================
        // DATA TABLE
        let mut data_table = Table::new();
        data_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        let data_title = "\x1b[34m\x1b[48;5;15mFirst Data\x1b[0m"; // Blue color
        let mut data_title_cell = Cell::new(data_title).with_hspan(2);
        data_title_cell.align(prettytable::format::Alignment::CENTER);
        data_table.add_row(Row::new(vec![data_title_cell]));

        data_table.add_row(Row::new(vec![Cell::new("Index"), Cell::new("Data")]));
        for idx in 0..5 {
            data_table.add_row(Row::new(vec![
                Cell::new(&format!("{}", idx)),
                Cell::new(&format!("0x{:016x}", self.data[idx])),
            ]));
        }
        data_table.printstd();

        // ===============================================================================
        // REGISTERS TABLE
        let mut regs_table = Table::new();
        regs_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        let regs_title = "\x1b[34m\x1b[48;5;15mFirst Registers\x1b[0m"; // Blue color
        let mut regs_title_cell = Cell::new(regs_title).with_hspan(10);
        regs_title_cell.align(prettytable::format::Alignment::CENTER);
        regs_table.add_row(Row::new(vec![regs_title_cell]));

        // First row with registers from 0 to 10
        regs_table.add_row(Row::new(
            (0..10).map(|i| Cell::new(&format!("Reg[{}]", i))).collect(),
        ));
        regs_table.add_row(Row::new(
            (0..10)
                .map(|i| Cell::new(&format!("0x{:08x}", self.regs[i])))
                .collect(),
        ));

        // Second row with registers from 10 to 20
        regs_table.add_row(Row::new(
            (10..20)
                .map(|i| Cell::new(&format!("Reg[{}]", i)))
                .collect(),
        ));
        regs_table.add_row(Row::new(
            (10..20)
                .map(|i| Cell::new(&format!("0x{:08x}", self.regs[i])))
                .collect(),
        ));
        regs_table.printstd();

        // ===============================================================================
        // DISPLAY IP
        println!("Instruction Pointer: {}", self.ip);
    }
}
