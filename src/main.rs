#![allow(non_snake_case, unused_variables, dead_code)]

use std::io; // stdin(), stdout()
use std::io::Write; // flush()
use unit_enum::UnitEnum; // UnitEnum, from_ordinal()

type Byte  = u8;
type Imm8  = u8;
type Word  = u16;
type Imm16 = u16;
type Addr  = u16;

#[derive(Debug, UnitEnum)]
enum Register {
    R0, R1, R2, R3
}

#[derive(Debug)]
enum Instruction {
    LOADI(Register,Imm16),
    LOAD(Register,Addr),
    NOP,
}

struct CPU {
    pc: u16,
    sp: u16,
    registers: [u8; 4],
    flags: u8,
}

struct RAM {
    mem: [u8; 65536], // 64KiB
}

struct Machine {
    cpu: CPU,
    ram: RAM,
    halted: bool,
}

impl CPU {
    fn new() -> Self {
        CPU{
            pc: 0x0000,
            sp: 0x0000, // !!! CHECK MEMORY MAP SPECIFICATION !!!
            registers: [0; 4],
            flags: 0b00000000
        }
    }

    fn reset(&mut self) {
        self.pc = 0x0000;
        self.sp = 0x0000; // !!! CHECK MEMORY MAP SPECIFICATION !!!
        self.registers = [0; 4];
        self.flags = 0b00000000;
    }
}

impl RAM {
    fn new() -> Self {
        RAM{ mem: [0; 65536] }
    }

    fn reset(&mut self)  {
        self.mem = [0; 65536];
    }

    fn read(&self, addr:Addr) -> Byte {
        self.mem[addr as usize]
    }
}

impl Machine {
    fn new() -> Self {
        Machine{ cpu: CPU::new(), ram: RAM::new(), halted: true }
    }

    fn start(&mut self) {
        self.cpu.reset();
        self.ram.reset();
        // self.load_program();
        self.halted = false;
    }

    fn log_state(&self) -> String {
        format!(
"
=====================================
PC = 0x{pc:0>6X}, SP = 0x{sp:0>6X}
Flags = {flags:b}
+----+----+----+----+
| R0 | R1 | R2 | R3 |
+----+----+----+----+
| {r0:0>2x} | {r1:0>2x} | {r2:0>2x} | {r3:0>2x} |
+----+----+----+----+
Cycles executed: ???
=====================================",
            pc=self.cpu.pc,
            sp=self.cpu.sp,
            r0=self.cpu.registers[0],
            r1=self.cpu.registers[1],
            r2=self.cpu.registers[2],
            r3=self.cpu.registers[3],
            flags=self.cpu.flags
        )
    }

    fn log_memory(&self) -> String {
        format!("NOPE")
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    fn fetch(&mut self) -> [Byte;3] {
        self.cpu.pc += 3;
        let pc = &self.cpu.pc;
        let ram = &self.ram;
        return [ram.read(pc-3), ram.read(pc-2), ram.read(pc-1)];
    }

    fn decode(&self, bytes: [Byte;3]) -> Instruction {
        let opcode: u8     = bytes[0] >> 2;
        let argA: Register = Register::from_ordinal((bytes[0] & 0b000011) as usize).unwrap();
        let argBlo: Byte   = bytes[1];
        let argBhi: Byte   = bytes[2];
        // let reg_reg: bool = ((opcode & 0b100000) == 0b100000);
        match opcode {
            0b100001 => Instruction::LOADI(argA, (((argBlo as u16) << 8) | argBhi as u16) as Imm16),
            0b100010 => Instruction::LOAD(argA, (((argBlo as u16) << 8) | argBhi as u16) as Addr),
            _ => Instruction::NOP,
        }
    }

    fn step(&mut self) {
        let instr_bytes = self.fetch();
        let instr = self.decode(instr_bytes);
        println!("{:?}",instr);
        // !!! EXECUTE INSTRUCTION !!!
    }
}

enum RunMode {
    Continuous,
    Step,
    Halted,
}

fn process_step_input(input: &String, machine: &Machine, mode: &mut RunMode) {
    match input.as_str().trim() {
        "s" | "step" => { },
        "c" | "continue" => {
            *mode = RunMode::Continuous;
        },
        _ => { println!("[ERROR]: couldn't recognize command."); }
    };
}

fn main() {
    let mut mode = RunMode::Step;
    let mut aster8: Machine = Machine::new();
    aster8.start();

    aster8.ram.mem[0] = 0b10000100;
    aster8.ram.mem[1] = 0b11111111;
    aster8.ram.mem[2] = 0b01010101;

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // let bytes: [u8;3] = [0b11111100, 0b0, 0b0];
    // println!("{:b}", bytes[0] << 2);
    // return;

    while !aster8.halted {
        match mode {
            RunMode::Continuous => {
                if aster8.halted == true { mode = RunMode::Halted; }
                aster8.step();
            },
            RunMode::Step => {
                buffer.clear();
                print!("{}\ndbg> ", aster8.log_state());
                stdout.flush().unwrap();
                stdin.read_line(&mut buffer).unwrap(); // !!! HANDLE RESULT !!!
                process_step_input(&buffer, &aster8, &mut mode);
                aster8.step();
            },
            RunMode::Halted => {
                aster8.halt();
            },
        };
    }
}
