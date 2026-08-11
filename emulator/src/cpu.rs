use crate::memory::Memory;
use std::{thread::sleep, time::Duration};

#[derive(Debug, PartialEq)]
pub struct Cpu {
    halt: bool,
    pc: u32,
    regs: RegisterFile,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            halt: false,
            pc: 0,
            regs: RegisterFile::new(),
        }
    }

    pub fn tick(&mut self, memory: &Memory) {
        if self.halt {
            sleep(Duration::from_micros(1));
            return;
        }
        self.execute(self.decode(memory.read_u32(self.pc)));
        //println!("{:#?}", self);
    }

    fn decode(&self, instruction: u32) -> Instruction {
        if instruction & 0b11 == 0b11 {
            return Instruction::Bits32(self.decode32(instruction));
        }
        panic!("only 32-bit wide instructions are supported");
    }

    fn decode32(&self, instruction: u32) -> Instruction32 {
        let real_op = ((instruction >> 2) & 0b1111) as u8;
        match real_op {
            0b0000 => Instruction32::Immediate {
                imm: ((instruction >> 20) & 0b111111111111) as u16,
                f4: ((instruction >> 16) & 0b1111) as u8,
                rs1: ((instruction >> 11) & 0b11111) as RegisterIdx,
                rd: ((instruction >> 6) & 0b11111) as RegisterIdx,
                op: (instruction & 0b111111) as u8,
            },
            0b0001 => Instruction32::Register {
                f7: ((instruction >> 25) & 0b1111111) as u8,
                f3: ((instruction >> 22) & 0b111) as u8,
                rs2: ((instruction >> 17) & 0b11111) as RegisterIdx,
                rs1: ((instruction >> 11) & 0b11111) as RegisterIdx,
                rd: ((instruction >> 6) & 0b11111) as RegisterIdx,
                op: (instruction & 0b111111) as u8,
            },
            0b0010 => Instruction32::LargeImmediate {
                imm: ((instruction >> 12) & 0b11111111111111111111),
                f1: ((instruction >> 11) & 0b1) != 0,
                rd: ((instruction >> 6) & 0b11111) as u8,
                op: (instruction & 0b111111) as u8,
            },
            _ => todo!(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        println!("{:#?}", instruction);
        match instruction {
            Instruction::Bits32(i) => self.execute32(i),
        }
    }

    fn execute32(&mut self, instruction: Instruction32) {
        match instruction {
            Instruction32::Immediate {
                imm,
                f4,
                rs1,
                rd,
                op,
            } => self.execute32i(imm, f4, rs1, rd, op),
            Instruction32::Register {
                f7,
                f3,
                rs2,
                rs1,
                rd,
                op,
            } => self.execute32r(f7, f3, rs2, rs1, rd, op),
            Instruction32::LargeImmediate { imm, f1, rd, op } => self.execute32l(imm, f1, rd, op),
        }
        self.pc += 4;
    }

    fn execute32i(&mut self, imm: u16, f4: u8, rs1: RegisterIdx, rd: RegisterIdx, op: Opcode32) {
        match op {
            0b000011 => {
                self.regs.write(rd, self.regs.read(rs1) + (imm as u32));
            }
            _ => todo!(),
        }
    }

    fn execute32r(
        &mut self,
        f7: u8,
        f3: u8,
        rs2: RegisterIdx,
        rs1: RegisterIdx,
        rd: RegisterIdx,
        op: Opcode32,
    ) {
        match op {
            0b000111 => {
                self.halt = true;
            }
            _ => todo!(),
        }
    }

    fn execute32l(&mut self, imm: u32, f1: bool, rd: RegisterIdx, op: Opcode32) {
        match op {
            0b001011 => {
                self.regs.write(rd, imm << if f1 { 0 } else { 12 });
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Bits32(Instruction32),
}

#[derive(Debug)]
enum Instruction32 {
    Register {
        f7: u8,           // [31..25]
        f3: u8,           // [24..22]
        rs2: RegisterIdx, // [21..17]
        rs1: RegisterIdx, // [15..11]
        rd: RegisterIdx,  // [10..06]
        op: Opcode32,     // [05..00]
    },
    Immediate {
        imm: u16,         // [31..20]
        f4: u8,           // [19..16]
        rs1: RegisterIdx, // [15..11]
        rd: RegisterIdx,  // [10..06]
        op: Opcode32,     // [05..00]
    },
    LargeImmediate {
        imm: u32,        // [31..12]
        f1: bool,        // [11..11]
        rd: RegisterIdx, // [10..06]
        op: Opcode32,    // [05..00]
    },
}

type RegisterIdx = u8;

type Opcode32 = u8;

#[derive(Debug, PartialEq)]
struct RegisterFile {
    inner: [u32; 32],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self { inner: [0; 32] }
    }

    pub fn read(&self, idx: RegisterIdx) -> u32 {
        self.inner[idx as usize]
    }

    pub fn write(&mut self, idx: RegisterIdx, val: u32) {
        let idx = idx as usize;
        if idx == 0 {
            return;
        }
        self.inner[idx] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hlt() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new(64);
        memory.write_at(
            0,
            &bytemuck::cast_slice(&[0b00000000_000_00000_00000_00000_000111]),
        );
        cpu.tick(&memory);
        cpu.tick(&memory);

        let mut expected = Cpu::new();
        expected.pc = 4;
        expected.halt = true;

        assert_eq!(cpu, expected);
    }

    #[test]
    fn addi() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new(64);
        memory.write_at(
            0,
            &bytemuck::cast_slice(&[0b000000101010_0000_00000_00010_000011]),
        );
        cpu.tick(&memory);

        let mut expected = Cpu::new();
        expected.pc = 4;
        expected.regs.write(2, 42);

        assert_eq!(cpu, expected);
    }

    #[test]
    fn movui() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new(64);
        memory.write_at(
            0,
            &bytemuck::cast_slice(&[0b000000000000000001_0_00010_001011]),
        );
        cpu.tick(&memory);

        let mut expected = Cpu::new();
        expected.regs.write(2, 0b00000000000000000001000000000000);
        expected.pc = 4;

        assert_eq!(cpu, expected);
    }

    #[test]
    fn movi() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new(64);
        memory.write_at(
            0,
            &bytemuck::cast_slice(&[0b000000000000000001_1_00010_001011]),
        );
        cpu.tick(&memory);

        let mut expected = Cpu::new();
        expected.regs.write(2, 0b00000000000000000000000000000001);
        expected.pc = 4;

        assert_eq!(cpu, expected);
    }

    #[test]
    #[should_panic]
    fn non_32_bit_instruction() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new(64);
        memory.write_at(
            0,
            &bytemuck::cast_slice(&[0b00000000000000000000000000000000]),
        );
        cpu.tick(&memory);
    }

    #[test]
    fn write_to_r0() {
        let mut cpu = Cpu::new();
        cpu.regs.write(0, 42);
    }
}
