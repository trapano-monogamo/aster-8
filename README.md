# Aster-8 CPU Specification (v0.1)

Von Neumann architecture for a CPU with 4 general purpose 8-bit registers and a 64KiB memory
device.

## Data Types

| Name | Size |
|-|-|
| Reg  | 2bit  |
| Byte | 8bit  |
| Word | 16bit |

## Memory Model

The address space is `0x0000-0xFFFF` with a simple memory map:

| Address Range | Purpose |
|-|-|
| `0x0000-0x0FFF` | text, data |
| `0x0FFF-0xFFFF` | stack |

All memory is uniform RAM.

## CPU State

Content:

| Register | Size | Purpose |
|-|-|-|
| PC | 16bit | program counter |
| SP | 16bit | stack pointer |
| R0 | 8bit | general purpose |
| R1 | 8bit | general purpose |
| R2 | 8bit | general purpose |
| R3 | 8bit | general purpose |
| Flags | 8bit | ZCNO: zero, carry, negative, overflow |

Reset state:

```
PC = 0x0000
SP = 0x????
R0,R1,R2,R3 = 0b00000000
Flags = 0b00000000
```

## Instruction Format

We'll use 3-bytes fixed-width instructions. The first byte is opcode (6 bits) + operand A (2
bits), the second is operand B low byte, and the third is operand B high byte. Operand A is
just two bits because it will always be one of the four general purpose registers.
```
0000 0000 . 0000 0000 . 0000 0000
^-----^     ^-------^   ^-------^
|      ^^   |      ^^   |
|opcode|    |      |    |
       |A   |      |    |
            |B_lo  |    |B_hi
                   |B_reg
```
When an instruction takes on register and one immediate/address then the operand B is
treated as a single byte (only low) or word (low and high), when an instruction takes two
registers as arguments, then the low byte of B is interpreted as a register (with padding) and
the high byte is discarded.

The first bit of the opcode signals whether the instruction has register-register arguments.

| Register | Bits |
|-|-|
| R0 | 00 |
| R1 | 01 |
| R2 | 10 |
| R3 | 11 |

## Instruction Set

Data movement:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| LOADI | 0b000001 | reg | imm16  | ??? | loads B to A | ??? |
| LOAD  | 0b000010 | reg | addr16 | ??? | loads B to A | ??? |
| STORE | 0b000011 | reg | addr16 | ??? | store A to addr B | ??? |
| MOVE  | 0b100100 | reg | reg  | ??? | moves content of A to B | ??? |

Arithmetic:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| ADDI | 0b000101 | reg | imm8 | ??? | adds B to A | ??? |
| ADDR | 0b100110 | reg | reg  | ??? | adds B to A | ??? |
| SUBI | 0b000111 | reg | imm8 | ??? | subtracts B from A | ??? |
| SUBR | 0b101000 | reg | reg  | ??? | subtracts B from A | ??? |
| INC  | 0b001001 | reg |      | ??? | increments A | ??? |
| DEC  | 0b001010 | reg |      | ??? | decrements A | ??? |

Logic:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| AND | 0b101011 | reg | reg | ??? | A & B | ??? |
| OR  | 0b101100 | reg | reg | ??? | A \| B | ??? |
| XOR | 0b101101 | reg | reg | ??? | A ^ B | ??? |
| NOT | 0b001110 | reg |     | ??? | sets A to ~B | ??? |

Comparison:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| CMPR | 0b101111 | reg | reg   | ??? | sets flags from A-B | ??? |
| CMPI | 0b010000 | reg | imm8  | ??? | sets flags from A-B | ??? |
| CMPA | 0b010001 | reg | add16 | ??? | sets flags from A-B | ??? |

Control flow:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| JMP  | 0b010010 | reg | addr16 | ??? | sets PC to B | ??? |
| JZ   | 0b010011 |     | addr16 | ??? | sets PC to B if Z=1 (i.e. if zero flag is true) | ??? |
| JNZ  | 0b010100 |     | addr16 | ??? | sets PC to B if Z=0 (i.e. if zero flag is false) | ??? |
| JC   | 0b010101 |     | addr16 | ??? | sets PC to B if C=1 (i.e. if carry flag is true) | ??? |
| JN   | 0b010110 |     | addr16 | ??? | sets PC to B if N=1 (i.e. if negative flag is true) | ??? |
| CALL | 0b010111 | ??? | ???    | ??? | ??? | ??? |
| RET  | 0b011000 | ??? | ???    | ??? | ??? | ??? |

System:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| NOP  | 0b011001 | | | ??? | no operation | ??? |
| HALT | 0b011010 | | | ??? | halts execution | ??? |
