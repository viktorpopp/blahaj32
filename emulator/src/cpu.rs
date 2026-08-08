use crate::memory::Memory;

#[derive(Debug)]
pub struct Cpu {
    pc: u32,
    registers: [u32; 32],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            registers: [0; 32],
        }
    }

    pub fn tick(&mut self, memory: &Memory) {
        self.execute(self.decode(memory.read_u32(self.pc)));
        println!("{:#?}", self);
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
                imm: ((instruction >> 16) & 0b1111111111111111) as u16,
                rs: ((instruction >> 11) & 0b11111) as Register,
                rd: ((instruction >> 6) & 0b11111) as Register,
                op: (instruction & 0b111111) as u8,
            },
            _ => todo!(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Bits32(i) => self.execute32(i),
        }
    }

    fn execute32(&mut self, instruction: Instruction32) {
        match instruction {
            Instruction32::Immediate { imm, rs, rd, op } => self.execute32i(imm, rs, rd, op),
        }
    }

    fn execute32i(&mut self, imm: u16, rs: Register, rd: Register, op: Opcode32) {
        match op {
            0b000011 => {
                self.registers[rd as usize] = self.registers[rs as usize] + (imm as u32);
                self.pc += 4;
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
    Immediate {
        imm: u16,
        rs: Register,
        rd: Register,
        op: Opcode32,
    },
}

type Register = u8;

type Opcode32 = u8;
