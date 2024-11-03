# *Simp*le em*ulator*

A simple emumator. We will create our own ISA with few instructions and emulate them.

## ISA

We will start by creating a simple, fixed-size instruction set architecture (ISA). It
is a great way to start understanding virtual machine design because the basic set of
instructions encoding scheme is essential for the emulator.

Our simple machine will have few registers, some RAM and a ALU for arithmetic operations.
We will need **LOAD** and **STORE** to transfer data from memory to register and the
other way. We will also have a **MOVE** instruction to manipulate immediates. And we
will have the **ADD** and **SUB** arithmetic operations.

So we will have 5 operations. But as we probably add more we will use more bits to have
room for adding them.

### Basic Structure of an 8-byte Instruction

To keep things simple, let’s design the 8-byte (64-bit) instruction with the following fields:

1. **Opcode (8 bits)**: Specifies the type of instruction (e.g., load, store, add, etc.).
2. **Destination Register (4 bits)**: Indicates the register to write to or read from.
3. **Source Register 1 (4 bits)**: Indicates a source register for the operation.
4. **Source Register 2 / Immediate Flag (4 bits)**: Specifies a second source register for
two-register operations or an immediate value flag.
5. **Immediate Value / Address (32 bits)**: Encodes a memory address for load/store instructions
or an immediate value for arithmetic operations.

This structure fits within 64 bits and leaves room for several types of operations while
providing flexibility for both immediate values and register-based operations.

### Instruction Set

Let’s define a basic set of instructions that operates on 4 general-purpose registers
(`R0` to `R3`) and perform the following operations:

1. **LOAD**: Load a value from memory into a register.
2. **STORE**: Store a value from a register into memory.
3. **ADD**: Add two registers or add a register and an immediate value.
4. **SUB**: Subtract one register from another or subtract an immediate value from a register.
5. **MOV**: Move an immediate value into a register.

### Encoding Scheme

For simplicity, let’s assume:
- **Registers** are encoded in 4 bits, allowing us to use values `0000` to `0011` for
registers `R0` to `R3`.
- **Opcodes** are encoded in 8 bits, allowing up to 256 different instructions
(we’ll only use a few for now).
- **Immediate/Address field** is 32 bits.

Here’s how each instruction looks in binary:

1. **LOAD (opcode 0x01)**
   Format: `LOAD dest_reg, [address]`
   - **Opcode**: `00000001`
   - **dest_reg**: Destination register to load data into (4 bits).
   - **source_reg_1**: Not used (set to `0000`).
   - **source_reg_2/immediate_flag**: Not used (set to `0000`).
   - **Immediate/Address**: Memory address (32 bits).
   - **Example**: `LOAD R1, [0x00400000]`
     ```
     Binary: 00000001 0001 0000 0000 000000000100000000000000
     ```

2. **STORE (opcode 0x02)**
   Format: `STORE [address], source_reg`
   - **Opcode**: `00000010`
   - **dest_reg**: Not used (set to `0000`).
   - **source_reg_1**: Register containing data to store (4 bits).
   - **source_reg_2/immediate_flag**: Not used (set to `0000`).
   - **Immediate/Address**: Memory address (32 bits).
   - **Example**: `STORE [0x00400004], R2`
     ```
     Binary: 00000010 0000 0010 0000 000000000100000000000100
     ```

3. **ADD (opcode 0x03)**
   Format: `ADD dest_reg, src_reg_1, src_reg_2`
   - **Opcode**: `00000011`
   - **dest_reg**: Destination register for the result (4 bits).
   - **source_reg_1**: First operand register (4 bits).
   - **source_reg_2/immediate_flag**: Second operand register (4 bits).
   - **Immediate/Address**: Not used (set to `00000000`).
   - **Example**: `ADD R0, R1, R2`
     ```
     Binary: 00000011 0000 0001 0010 000000000000000000000000
     ```

4. **SUB (opcode 0x04)**
   Format: `SUB dest_reg, src_reg_1, src_reg_2`
   - **Opcode**: `00000100`
   - **dest_reg**: Destination register for the result (4 bits).
   - **source_reg_1**: First operand register (4 bits).
   - **source_reg_2/immediate_flag**: Second operand register (4 bits).
   - **Immediate/Address**: Not used (set to `00000000`).
   - **Example**: `SUB R1, R2, R3`
     ```
     Binary: 00000100 0001 0010 0011 000000000000000000000000
     ```

5. **MOV (opcode 0x05)**
   Format: `MOV dest_reg, immediate_value`
   - **Opcode**: `00000101`
   - **dest_reg**: Destination register (4 bits).
   - **source_reg_1**: Not used (set to `0000`).
   - **source_reg_2/immediate_flag**: Immediate flag (set to `0001` to indicate immediate mode).
   - **Immediate/Address**: Immediate value to load (32 bits).
   - **Example**: `MOV R2, 0x0000000A`
     ```
     Binary: 00000101 0010 0000 0001 00000000000000000000001010
     ```

### Summary Table

Here is a simple assembly code:
```asm
LOAD R1, [0x00400000]
STORE [0x00400000], R2
ADD R0, R1, R2
SUB R1, R2, R3
MOVE R2, 0x00400004
```

| Instruction | Binary Representation                          | Description                              |
|-------------|-----------------------------------------------|------------------------------------------|
| `LOAD R1, [0x00400000]` | `00000001 0001 0000 0000 000000000100000000000000` | Load from memory address into `R1`      |
| `STORE [0x00400004], R2`| `00000010 0000 0010 0000 000000000100000000000100` | Store `R2` to memory address            |
| `ADD R0, R1, R2`        | `00000011 0000 0001 0010 000000000000000000000000` | Add `R1` and `R2`, store in `R0`        |
| `SUB R1, R2, R3`        | `00000100 0001 0010 0011 000000000000000000000000` | Subtract `R3` from `R2`, store in `R1`  |
| `MOV R2, 0x0000000A`    | `00000101 0010 0000 0001 00000000000000000000001010` | Load immediate `0x0000000A` into `R2`   |

### Explanation of Each Field

- **Opcode**: Identifies the instruction type.
- **Destination Register (dest_reg)**: Register to store the result of the operation.
- **Source Register 1 (source_reg_1)**: Primary source register for the instruction.
- **Source Register 2 / Immediate Flag**: Either a second source register or a flag for immediate mode.
- **Immediate/Address**: Memory address for load/store instructions or an immediate value for arithmetic operations.

This simple setup is enough to load values into registers, store values from registers into memory, and perform basic arithmetic operations. It gives you a foundation for more complex instructions later on while keeping encoding straightforward.
