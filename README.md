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
the high byte is discarded. The encoding is as follows

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
| LOADI | ??? | reg | imm16  | ??? | loads B to A | ??? |
| LOAD  | ??? | reg | addr16 | ??? | loads B to A | ??? |
| STORE | ??? | reg | addr16 | ??? | store A to addr B | ??? |
| MOVE  | ??? | reg | reg  | ??? | moves content of A to B | ??? |

Arithmetic:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| ADDI | ??? | reg | imm8 | ??? | adds B to A | ??? |
| ADDR | ??? | reg | reg  | ??? | adds B to A | ??? |
| SUBI | ??? | reg | imm8 | ??? | subtracts B from A | ??? |
| SUBR | ??? | reg | reg  | ??? | subtracts B from A | ??? |
| INC  | ??? | reg |      | ??? | increments A | ??? |
| DEC  | ??? | reg |      | ??? | decrements A | ??? |

Logic:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| AND | ??? | reg | reg | ??? | A & B | ??? |
| OR  | ??? | reg | reg | ??? | A \| B | ??? |
| XOR | ??? | reg | reg | ??? | A ^ B | ??? |
| NOT | ??? | reg |     | ??? | sets A to ~B | ??? |

Comparison:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| CMPR | ??? | reg | reg   | ??? | sets flags from A-B | ??? |
| CMPI | ??? | reg | imm8  | ??? | sets flags from A-B | ??? |
| CMPA | ??? | reg | add16 | ??? | sets flags from A-B | ??? |

Control flow:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| JMP  | ??? | reg | addr16 | ??? | sets PC to B | ??? |
| JZ   | ??? |     | addr16 | ??? | sets PC to B if Z=1 (i.e. if zero flag is true) | ??? |
| JNZ  | ??? |     | addr16 | ??? | sets PC to B if Z=0 (i.e. if zero flag is false) | ??? |
| JC   | ??? |     | addr16 | ??? | sets PC to B if C=1 (i.e. if carry flag is true) | ??? |
| JN   | ??? |     | addr16 | ??? | sets PC to B if N=1 (i.e. if negative flag is true) | ??? |
| CALL | ??? | ??? | ???    | ??? | ??? | ??? |
| RET  | ??? | ??? | ???    | ??? | ??? | ??? |

System:

| Mnemonic | Opcode | A | B | Cycles | Description | Flags |
|-|-|-|-|-|-|-|
| NOP  | ??? | | | ??? | no operation | ??? |
| HALT | ??? | | | ??? | halts execution | ??? |
