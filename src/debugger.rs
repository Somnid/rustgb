use super::gameboy::*;
use super::cpu;
use std::io::{stdin, stdout, Write};

enum DebugCommand {
	Step,
	WaitForLocation(u16)
}

pub struct Debugger<'a> {
	gameboy: &'a mut Gameboy
}

impl<'a> Debugger<'a> {
	pub fn new(gameboy: &'a mut Gameboy) -> Self {
		Debugger {
			gameboy: gameboy
		}
	}

	pub fn run(&mut self){
		loop {
			print!("rust-gb>");
			stdout().flush().unwrap();

			let input = read_stdin();
			let command = self.parse_input(input.as_ref());
			match command {
				DebugCommand::Step => self.step(),
				DebugCommand::WaitForLocation(loc) => {
					println!("Wait for location is not implemented, stepping...");
					self.step()
				}
			}
		}
	}

	pub fn step(&mut self){
		let step_result = self.gameboy.step();
		println!("Stepped to {}", step_result.pc);
	}

	fn parse_input(&self, input: &str) -> DebugCommand {
		let command_char = input.chars().nth(0);
		match command_char {
			Some('w') => {
				match input.chars().skip(1).collect::<String>().trim().parse::<u16>() {
					Ok(r) => DebugCommand::WaitForLocation(r),
					Err(_) => DebugCommand::Step
				}
			},
			Some(_) => {
				DebugCommand::Step
			},
			None => {
				DebugCommand::Step
			}
		}
	}

	fn get_description_for_cpu_result(result: cpu::CpuStepResult){
		match result.opcode {
			_ => println!("TODO: Add descriptions here")
		}
	}
}

fn read_stdin() -> String {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input.trim().into()
}
