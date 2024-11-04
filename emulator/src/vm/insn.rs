#[derive(Clone, Copy)]
enum Opcode {
    Load = 1,
    Store,
    Move,
    Add,
    Sub,
}

impl Opcode {
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

fn parse_number(input: &str) -> Result<i32, String> {
    if input.starts_with("0x") {
        // Hexadecimal
        let stripped = input
            .strip_prefix("0x")
            .ok_or_else(|| format!("Invalid hex format: {}", input))?;
        i32::from_str_radix(stripped, 16).map_err(|e| e.to_string())
    } else if input.starts_with("0b") {
        // Binary
        let stripped = input
            .strip_prefix("0b")
            .ok_or_else(|| format!("Invalid binary format: {}", input))?;
        i32::from_str_radix(stripped, 2).map_err(|e| e.to_string())
    } else {
        // Decimal
        input.parse::<i32>().map_err(|e| e.to_string())
    }
}

pub fn decode(s: &str) -> u64 {
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

    let mut decoding: u64 = ((opcode as u64) << 56) as u64;

    // Depending of the opcode we will decode different kind of information
    match opcode {
        Opcode::Load => todo!("Need to decode load operands"),
        Opcode::Store => todo!("Need to decode store operands"),
        Opcode::Move => {
            // First we expect an immediate
            let imm_str = insns.next().unwrap(); // We are expecting a u32
            let imm = match parse_number(imm_str) {
                Err(e) => panic!("Failed to get immediate from {}: {}", s, e),
                Ok(v) => v,
            };
            // Then expect a register as destination
            let reg_str = insns.next().unwrap(); // We should have a string starting by 'r'
            let reg_str = reg_str.to_lowercase();
            let num_reg = match reg_str.trim_start_matches('r').parse::<u8>() {
                Ok(num) => num,
                Err(_) => panic!("Failed to get register from {}", s),
            };

            println!(
                "Decoded {}: MOVE Immediate {}, Register {}",
                s, imm, num_reg
            );
            decoding |= (((num_reg as u64) << 32) as u64) | (imm as u64)
        }
        Opcode::Add => todo!("Need to decode add operands"),
        Opcode::Sub => todo!("Need to decode sub operands"),
    }

    // for s in it {
    //     println!("decoding {s}");
    // }
    decoding
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_load_insn() {
        let result: u64 = decode("load 0x1234 r1");
        let expected: u64 = 0b00000001_00000000_00000000_00000001_00000000000000000000010011010010;
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_store_insn() {
        let result: u64 = decode("store r1 0x1234");
        let expected: u64 = 0b00000010_00000001_00000000_00000000_00000000000000000000010011010010;
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_move_lowercase_insn() {
        let result: u64 = decode("move 0xbad r120");
        let expected: u64 = 0b00000011_00000000_00000000_01111000_00000000000000000000101110101101;
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_move_mixedcase_insn() {
        let result: u64 = decode("Move 0xBAD r120");
        let expected: u64 = 0b00000011_00000000_00000000_01111000_00000000000000000000101110101101;
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_move_tabs_spaces_insn() {
        let result: u64 = decode("\t   move\t0xbad       r120");
        let expected: u64 = 0b00000011_00000000_00000000_01111000_00000000000000000000101110101101;
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_add_insn() {
        let result: u64 = decode("add r10 r20 r42");
        let expected: u64 = 0b00000100_00001010_00010100_00101010_00000000000000000000000000000000;
        assert_eq!(result, expected);
    }

    #[test]
    fn testing_sub_insn() {
        let result: u64 = decode("sub r10 r20 r42");
        let expected: u64 = 0b00000101_00001010_00010100_00101010_00000000000000000000000000000000;
        assert_eq!(result, expected);
    }
}
