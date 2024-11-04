# *SIMP*le em*ULATOR*

A simple emumator. We will create our own ISA with few instructions and emulate them.

## Table of contents
- [ISA](#isa)
- [Structure of instruction](#basic-structure-of-an-8-byte-instruction)
- [Instruction Set](#instruction-set)
- [Encoding Scheme](#encoding-scheme)
- [Code example](#code-example)
- [Binary translation](#binary-translation)
- [How to run simulator](#how-to-run-simulator)

---

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

To keep things simple, let’s design the 8-bytes (64-bits) instruction with the following fields:

1. **Opcode (8 bits)**: Specifies the type of instruction (e.g., load, store, add, etc.).
1. **Source Register 1 (8 bits)**: Indicates a source register for the operation.
1. **Source Register 2 (8 bits)**: Specifies a second source register for two-register operations.
1. **Destination Register (8 bits)**: Indicates the register to write to or read from.
1. **Immediate Value / Address (32 bits)**: Encodes a memory address for load/store instructions
or an immediate value for the move instruction.

This structure fits within 64 bits and leaves room for several types of operations while
providing flexibility for both immediate values and register-based operations.

### Instruction Set

Let’s define a basic set of instructions that operates on 4 general-purpose registers
(`R0` to `R3`) and perform the following operations:

1. **LOAD**: Load a value from memory into a register.
1. **STORE**: Store a value from a register into memory.
1. **MOVE**: Move an immediate value into a register.
1. **ADD**: Add two registers and save the result into a third one
1. **SUB**: Subtract two registers and save the result into a third one

### Encoding Scheme

For simplicity, let’s assume:
- **Registers** are encoded in 8 bits, allowing us to use values `00` to `FF` for
registers `R0` to `R255`.
- **Opcodes** are encoded in 8 bits, allowing up to 256 different instructions
(we’ll only use a few for now).
- **Immediate/Address field** is 32 bits.

### Summary table

| Instruction       | Symbolic         | Description                            |
|-------------------|------------------|----------------------------------------|
| `LOAD 0x4321 Rx`  | Mem@0x4321 -> Rx | Load data at Mem[0x4321] into Rx       |
| `STORE Rx 0x1234` | Rx -> Mem@0x1234 | Store data in Rx into Mem[0x4321]      |
| `ADD Rx Ry Rz`    | Rx + Ry -> Rz    | Add Rx and Ry and store result into Rz |
| `SUB Rx Ry Rz`    | Rx - Ry -> Rz    | Sub Rx and Ry and store result into Rz |
| `MOV 0xCAFE Rx`   | 0xCAFE -> Rx     | Store the immediate 0xCAFE into Rx     |

### Code example

Here is a simple assembly code that added `2989` and `51966` and store the result at
address 0x1234 in memory:

```asm
MOVE 0xBAD  R0
MOVE 0xCAFE  R1
ADD R0  R1  R2
STORE R2  0x1234
```

We are expecting one instruction per line and each part of the instruction are separated
by one or more spaces or tabs.

### Binary translation

1. **LOAD (opcode 0x01)**
   Format: `LOAD address dest_reg`
   - **Opcode**: `00000001`
   - **source_reg_1**: Not used (set to `0`).
   - **source_reg_2**: Not used (set to `0`).
   - **dest_reg**: Destination register to load data into (8 bits).
   - **Immediate/Address**: Memory address (32 bits).
   - **Example**: `LOAD 0x00400000 R1`
     ```
     Binary: 00000001 00000000 00000000 00000001 00000000_00000000_00100000_00000000
     ```

1. **STORE (opcode 0x02)**
   Format: `STORE source_reg_1 address`
   - **Opcode**: `00000010`
   - **source_reg_1**: Register containing data to store (8 bits).
   - **source_reg_2**: Not used (set to `0`).
   - **dest_reg**: Not used (set to `0`).
   - **Immediate/Address**: Memory address (32 bits).
   - **Example**: `STORE R2, 0x00400004`
     ```
     Binary: 00000010 00000010 00000000 00000000 00000000_00000000_00100000_00000000
     ```

1. **MOVE (opcode 0x03)**
   Format: `MOVE immediate_value dest_reg`
   - **Opcode**: `00000011`
   - **source_reg_1**: Not used (set to `0`).
   - **source_reg_2**: Not used (set to `0`).
   - **dest_reg**: Destination register (8 bits).
   - **Immediate/Address**: Immediate value to load (32 bits).
   - **Example**: `MOVE 0xCAFEDECA, R2`
     ```
     Binary: 00000011 00000000 00000000 00000010 11001010_11111110_11011110_11001010
     ```

1. **ADD (opcode 0x04)**
   Format: `ADD src_reg_1 src_reg_2 dest_reg`
   - **Opcode**: `00000100`
   - **source_reg_1**: First operand register (8 bits).
   - **source_reg_2**: Second operand register (8 bits).
   - **dest_reg**: Destination register for the result (8 bits).
   - **Immediate/Address**: Not used (set to `0`).
   - **Example**: `ADD R0, R1, R2`
     ```
     Binary: 00000100 00000000 00000001 00000010 00000000_00000000_00000000_00000000
     ```

1. **SUB (opcode 0x05)**
   Format: `SUB src_reg_1 src_reg_2 dest_reg`
   - **Opcode**: `00000101`
   - **source_reg_1**: First operand register (8 bits).
   - **source_reg_2**: Second operand register (8 bits).
   - **dest_reg**: Destination register for the result (8 bits).
   - **Immediate/Address**: Not used (set to `0`).
   - **Example**: `SUB R1, R2, R3`
     ```
     Binary: 00000101 00000001 00000010 00000011 00000000_00000000_00000000_00000000
     ```

## How to run simulator?

- `cargo run`
