use super::cpu;
use super::interconnect;
use std::io::Write;
use std::io::Cursor;

pub struct Gameboy {
	interconnect : interconnect::Interconnect,
	cpu : cpu::Cpu
}

impl Gameboy {
	pub fn new(bootrom: &mut[u8]) -> Gameboy {
		Gameboy {
			interconnect : interconnect::Interconnect::new(bootrom),
			cpu : cpu::Cpu::new()
		}
	}
	pub fn run(&mut self){
		loop {
			self.cpu.step(&mut self.interconnect);
		}
	}
	pub fn step(&mut self) -> cpu::CpuStepResult {
		self.cpu.step(&mut self.interconnect)
	}
}
