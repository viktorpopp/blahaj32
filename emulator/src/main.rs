use crate::{cli::Cli, cpu::Cpu, memory::Memory};
use clap::Parser;
use color_eyre::eyre::{Result, eyre};
use std::fs;

mod cli;
mod cpu;
mod memory;

struct Emulator {
    cpu: Cpu,
    memory: Memory,
}

impl Emulator {
    pub fn new(config: Config) -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(config.memory_capacity.0),
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
    memory_capacity: MemoryCapacity,
}

impl TryFrom<Cli> for Config {
    type Error = String;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        let memory_capacity = match cli.memory {
            Some(m) => MemoryCapacity::try_from(m)?,
            None => MemoryCapacity(64),
        };

        Ok(Config { memory_capacity })
    }
}

pub struct MemoryCapacity(pub u32);

impl TryFrom<String> for MemoryCapacity {
    type Error = String;

    // TODO: This can be optimized.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut num_count = 0;
        for c in s.chars() {
            if c.is_numeric() {
                num_count += 1;
                continue;
            }
            break;
        }
        let num = &s[0..num_count];
        let num = num.parse::<u32>().map_err(|e| e.to_string())?;

        let unit = s.chars().find(|c| c.is_alphabetic()).unwrap_or('B');
        let unit_mul = match unit {
            'B' => 1,
            'K' => 1024,
            _ => todo!(),
        };

        Ok(MemoryCapacity(num * unit_mul))
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let config = Config::try_from(args.clone()).map_err(|e| eyre!(e))?;

    let program = fs::read(&args.file)?;

    let mut emulator = Emulator::new(config);
    emulator.load_at(0, bytemuck::cast_slice(&program));
    emulator.run();

    Ok(())
}
