#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Load = 1,
    Store,
    Move,
    Add,
    Sub,
}

impl Opcode {
    pub fn from_u64(value: u64) -> Option<Opcode> {
        match value {
            x if x == Opcode::Load as u64 => Some(Opcode::Load),
            x if x == Opcode::Store as u64 => Some(Opcode::Store),
            x if x == Opcode::Move as u64 => Some(Opcode::Move),
            x if x == Opcode::Add as u64 => Some(Opcode::Add),
            x if x == Opcode::Sub as u64 => Some(Opcode::Sub),
            _ => None,
        }
    }

    pub fn from_string(s: &str) -> Option<Opcode> {
        match s.to_lowercase().as_str() {
            "load" => Some(Opcode::Load),
            "store" => Some(Opcode::Store),
            "add" => Some(Opcode::Add),
            "sub" => Some(Opcode::Sub),
            "move" => Some(Opcode::Move),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Insn(u64);

impl Default for Insn {
    fn default() -> Self {
        Insn::new(0)
    }
}

impl Insn {
    const OPCODE_SHIFT: u8 = 56;
    const OPCODE_MASK: u64 = 0xFF000000_00000000;

    const SRC1_SHIFT: u8 = 48;
    const SRC1_MASK: u64 = 0x00FF0000_00000000;

    const SRC2_SHIFT: u8 = 40;
    const SRC2_MASK: u64 = 0x0000FF00_00000000;

    const DEST_SHIFT: u8 = 32;
    const DEST_MASK: u64 = 0x000000FF_00000000;

    const IMMEDIATE_SHIFT: u64 = 0;
    const IMMEDIATE_MASK: u64 = 0x00000000_FFFFFFFF;

    pub fn new(v: u64) -> Self {
        Insn(v)
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }

    fn set_opcode(&mut self, opcode: u8) {
        self.0 &= !Insn::OPCODE_MASK; // Clear opcode bits
        self.0 |= ((opcode as u64) << Insn::OPCODE_SHIFT) & Insn::OPCODE_MASK;
    }

    fn set_src1(&mut self, src1: u8) {
        self.0 &= !Insn::SRC1_MASK;
        self.0 |= ((src1 as u64) << Insn::SRC1_SHIFT) & Insn::SRC1_MASK;
    }

    fn set_src2(&mut self, src2: u8) {
        self.0 &= !Insn::SRC2_MASK;
        self.0 |= ((src2 as u64) << Insn::SRC2_SHIFT) & Insn::SRC2_MASK;
    }

    fn set_dest(&mut self, dest: u8) {
        self.0 &= !Insn::DEST_MASK;
        self.0 |= ((dest as u64) << Insn::DEST_SHIFT) & Insn::DEST_MASK;
    }

    fn set_immediate(&mut self, imm: u32) {
        self.0 &= !Insn::IMMEDIATE_MASK;
        self.0 |= ((imm as u64) << Insn::IMMEDIATE_SHIFT) & Insn::IMMEDIATE_MASK;
    }

    pub fn get_opcode(self) -> Option<Opcode> {
        Opcode::from_u64((self.0 & Insn::OPCODE_MASK) >> Insn::OPCODE_SHIFT)
    }

    pub fn get_src1(self) -> u8 {
        ((self.0 & Insn::SRC1_MASK) >> Insn::SRC1_SHIFT) as u8
    }

    pub fn get_src2(self) -> u8 {
        ((self.0 & Insn::SRC2_MASK) >> Insn::SRC2_SHIFT) as u8
    }

    pub fn get_dest(self) -> u8 {
        ((self.0 & Insn::DEST_MASK) >> Insn::DEST_SHIFT) as u8
    }

    pub fn get_immediate(self) -> u32 {
        ((self.0 & Insn::IMMEDIATE_MASK) >> Insn::IMMEDIATE_SHIFT) as u32
    }

    pub fn bin_translation(s: &str) -> Insn {
        let delimiters = [' ', '\t'];
        let mut insns = s
            .split(|c| delimiters.contains(&c))
            .filter(|s| !s.is_empty());

        let op_str = match insns.next() {
            None => panic!("Failed to find an opcode"),
            Some(op) => op,
        };

        let opcode = match Opcode::from_string(op_str) {
            None => panic!("Unkown opcode {}", op_str),
            Some(v) => v,
        };

        let mut insn_builder: Insn = Insn::default();
        insn_builder.set_opcode(opcode as u8);

        // Depending of the opcode we will decode different kind of information
        match opcode {
            Opcode::Load => {
                // Read the memory address
                let mem_str = insns.next().unwrap(); // We are expecting a u32
                let mem_addr = match parse_number(mem_str) {
                    Err(e) => panic!("Failed to read memory address from {}: {}", s, e),
                    Ok(v) => v,
                };
                insn_builder.set_immediate(mem_addr);

                // Then we should have a register
                let reg_str = insns.next().unwrap(); // We should have a string starting by 'r'
                let reg_str = reg_str.to_lowercase();
                let num_reg = match reg_str.trim_start_matches('r').parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => panic!("Failed to get register from {}", s),
                };
                insn_builder.set_dest(num_reg);

                println!(
                    "Decoded {}: LOAD Memory {}, Register {}",
                    s, mem_addr, num_reg
                );
            }
            Opcode::Store => {
                // We expect a register
                let reg_str = insns.next().unwrap(); // We should have a string starting by 'r'
                let reg_str = reg_str.to_lowercase();
                let num_reg = match reg_str.trim_start_matches('r').parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => panic!("Failed to get register from {}", s),
                };
                insn_builder.set_src1(num_reg);

                // Then read the memory address
                let mem_str = insns.next().unwrap(); // We are expecting a u32
                let mem_addr = match parse_number(mem_str) {
                    Err(e) => panic!("Failed to read memory address from {}: {}", s, e),
                    Ok(v) => v,
                };
                insn_builder.set_immediate(mem_addr);

                println!(
                    "Decoded {}: STORE Register {}, Memory {}",
                    s, num_reg, mem_addr
                );
            }
            Opcode::Move => {
                // First we expect an immediate
                let imm_str = insns.next().unwrap(); // We are expecting a u32
                let imm = match parse_number(imm_str) {
                    Err(e) => panic!("Failed to get immediate from {}: {}", s, e),
                    Ok(v) => v,
                };
                insn_builder.set_immediate(imm);

                // Then expect a register as destination
                let reg_str = insns.next().unwrap(); // We should have a string starting by 'r'
                let reg_str = reg_str.to_lowercase();
                let num_reg = match reg_str.trim_start_matches('r').parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => panic!("Failed to get register from {}", s),
                };
                insn_builder.set_dest(num_reg);

                println!(
                    "Decoded {}: MOVE Immediate {}, Register {}",
                    s, imm, num_reg
                );
            }
            Opcode::Add | Opcode::Sub => {
                // We are expected three registers
                let src1_str = insns.next().unwrap(); // We should have a string starting by 'r'
                let src1_str = src1_str.to_lowercase();
                let src1_reg = match src1_str.trim_start_matches('r').parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => panic!("Failed to get source1 register from {}", s),
                };
                insn_builder.set_src1(src1_reg);

                let src2_str = insns.next().unwrap(); // We should have a string starting by 'r'
                let src2_str = src2_str.to_lowercase();
                let src2_reg = match src2_str.trim_start_matches('r').parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => panic!("Failed to get source2 register from {}", s),
                };
                insn_builder.set_src2(src2_reg);

                let dest_str = insns.next().unwrap(); // We should have a string starting by 'r'
                let dest_str = dest_str.to_lowercase();
                let dest_reg = match dest_str.trim_start_matches('r').parse::<u8>() {
                    Ok(num) => num,
                    Err(_) => panic!("Failed to get destination register from {}", s),
                };
                insn_builder.set_dest(dest_reg);

                println!(
                    "Decoded {}: ADD Register {}, Register {}, Register {}",
                    s, src1_reg, src2_reg, dest_reg
                );
            }
        }

        insn_builder
    }
}

fn parse_number(input: &str) -> Result<u32, String> {
    if input.starts_with("0x") {
        // Hexadecimal
        let stripped = input
            .strip_prefix("0x")
            .ok_or_else(|| format!("Invalid hex format: {}", input))?;
        u32::from_str_radix(stripped, 16).map_err(|e| e.to_string())
    } else if input.starts_with("0b") {
        // Binary
        let stripped = input
            .strip_prefix("0b")
            .ok_or_else(|| format!("Invalid binary format: {}", input))?;
        u32::from_str_radix(stripped, 2).map_err(|e| e.to_string())
    } else {
        // Decimal
        input.parse::<u32>().map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_load_insn() {
        let result = Insn::bin_translation("load 0x1234 r1");
        let expected =
            Insn::new(0b00000001_00000000_00000000_00000001_00000000000000000001001000110100);
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_store_insn() {
        let result = Insn::bin_translation("store r1 0x1234");
        let expected =
            Insn::new(0b00000010_00000001_00000000_00000000_00000000000000000001001000110100);
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_move_lowercase_insn() {
        let result = Insn::bin_translation("move 0xbad r120");
        let expected =
            Insn::new(0b00000011_00000000_00000000_01111000_00000000000000000000101110101101);
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_move_mixedcase_insn() {
        let result = Insn::bin_translation("Move 0xBAD r120");
        let expected =
            Insn::new(0b00000011_00000000_00000000_01111000_00000000000000000000101110101101);
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_move_tabs_spaces_insn() {
        let result = Insn::bin_translation("\t   move\t0xbad       r120");
        let expected =
            Insn::new(0b00000011_00000000_00000000_01111000_00000000000000000000101110101101);
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_add_insn() {
        let result = Insn::bin_translation("add r10 r20 r42");
        let expected =
            Insn::new(0b00000100_00001010_00010100_00101010_00000000000000000000000000000000);
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_sub_insn() {
        let result = Insn::bin_translation("sub r10 r20 r42");
        let expected =
            Insn::new(0b00000101_00001010_00010100_00101010_00000000000000000000000000000000);
        assert_eq!(result, expected);
    }
}
