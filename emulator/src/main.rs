use crate::{cpu::Cpu, memory::Memory};

mod cpu;
mod memory;

const PROGRAM: [u32; 1] = [
    // addi r2, r0, 42
    // imm              rs    rd    op
    0b0000000000101010_00000_00010_000011,
];

struct Emulator {
    cpu: Cpu,
    memory: Memory,
}

impl Emulator {
    pub fn new(config: Config) -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(config.memory_capacity),
        }
    }

    pub fn load_at(&mut self, at: u32, data: &[u8]) {
        self.memory.write_at(at, data);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.tick(&self.memory);
        }
    }
}

struct Config {
    memory_capacity: u32,
}

fn main() {
    let config = Config {
        memory_capacity: 8, // 8 bytes of memory
    };
    let mut emulator = Emulator::new(config);
    emulator.load_at(0, &bytemuck::cast_slice(&PROGRAM));
    emulator.run();
}
