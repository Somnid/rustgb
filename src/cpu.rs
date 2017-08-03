use super::gameboy;
use super::interconnect::*;
use super::add_signed::*;

#[derive(Debug)]
struct FlagRegister {
	zero_flag : bool, //set on zero (Z)
	subtract_flag : bool, //set if subtraction occured (N)
	half_carry_flag: bool, //set if carry on lower 4-bits (H)
	carry_flag: bool //carry occured (C)
}

#[derive(Debug,Clone,Copy)]
enum Register{
	A,
	B,
	C,
	D,
	E,
	BC,
	DE,
	HL,
	SP
}

#[derive(Debug, Copy, Clone)]
enum Location {
	Reg(Register),
	Ram(u16)
}

pub struct CpuStepResult {
	pub pc : u16,
	pub opcode: u8
}

#[derive(Debug)]
pub struct Cpu {
	reg_a : u8,
	reg_b : u8,
	reg_c : u8,
	reg_d : u8,
	reg_e : u8,
	reg_h : u8,
	reg_l : u8,
	sp : u16, //16-bit
	pc : u16, //16-bit
	flag_reg : FlagRegister
}

impl Cpu {
	pub fn new() -> Cpu {
		Cpu {
			reg_a : 0,
			reg_b : 0,
			reg_c : 0,
			reg_d : 0,
			reg_e : 0,
			reg_h : 0,
			reg_l : 0,
			sp : 0,
			pc : 0,
			flag_reg : FlagRegister {
				zero_flag : false,
				subtract_flag : false,
				half_carry_flag: false,
				carry_flag: false
			}
		}
	}
	pub fn step(&mut self, interconnect: &mut Interconnect) -> CpuStepResult {
		let pc = self.pc;
		let opcode = interconnect.read_u8(pc);
		println!("Executing opcode {:#X} from location {:#X}", opcode, pc);
		match opcode {
			0x0E => self.load_u8(interconnect, Location::Reg(Register::C), Location::Ram(pc + 1), 1),
			0x20 => {
				self.pc += 1;
				let offset = interconnect.read_u8(self.pc) as i8;
				if !self.flag_reg.zero_flag {
					let new_address = self.pc.add_signed(offset) + 1; //address after this instruction is the current address
					self.pc = new_address;
					println!("took branch: JRNZ, {:?} to location {:?}", offset as i8, new_address);
				}else {
					self.pc += 1;
					println!("did not take branch: JRNZ, {:?}", offset as i8);
				}
			}
			0x21 => self.load_u16_immediate(interconnect, Location::Reg(Register::HL)),
			0x31 => self.load_u16_immediate(interconnect, Location::Reg(Register::SP)),
			0x32 => {
				self.pc += 1;
				let address = self.read_reg_u16(Register::HL);
				interconnect.write_u8(address, self.reg_a);
				let new_hl = address - 1;
				self.write_reg_u16(Register::HL, new_hl);
				println!("executed: LD(HL-), A");
			},
			0x3E => self.load_u8(interconnect, Location::Reg(Register::A), Location::Ram(pc + 1), 1),
			0xAF => {
				self.reg_a = self.reg_a ^ self.reg_a;
				self.flag_reg.zero_flag = self.reg_a == 0;
				self.flag_reg.subtract_flag = false;
				self.flag_reg.half_carry_flag = false;
				self.flag_reg.carry_flag = false;
				self.pc += 1;
				println!("executed: XOR A");
			}
			0xCB => self.read_extension_op(interconnect),
			0xE2 => {
				let offset_location = 0x00FF + self.reg_c as u16;
				self.load_u8(interconnect, Location::Ram(offset_location), Location::Reg(Register::A), 0)
			}
			_ => panic!("Don't know what to do with instruction: {:#X} at location {:#X}", opcode, self.pc)
		}

		CpuStepResult{
			pc : self.pc,
			opcode: opcode
		}
	}
	fn read_extension_op(&mut self, interconnect: &Interconnect){
		self.pc += 1;
		let extension_op = interconnect.read_u8(self.pc);
		match extension_op {
			0x7C => {
				self.flag_reg.zero_flag = (self.reg_h >> 7) == 0;
				self.flag_reg.half_carry_flag = true;
				self.flag_reg.subtract_flag = false;
				self.pc += 1;
				println!("executed: BIT7,H");
			}
			_ => panic!("unimplmented extension_op: {:#X}", extension_op)
		}
	}
	fn load_u16_immediate(&mut self, interconnect: &mut Interconnect, location: Location){
		self.pc += 1;
		let immediate = interconnect.read_u16(self.pc);
		match location {
			Location::Reg(reg) => self.write_reg_u16(reg, immediate),
			Location::Ram(address) => panic!("We don't know how to write u16s to RAM yet.")
		}
		self.pc += 2;
		println!("executed: LD {:?}, {:#X}", location, immediate);
	}
	fn load_u8(&mut self, interconnect: &mut Interconnect, location: Location, value_location: Location, pc_offset: u16){
		self.pc += 1;
		let value = self.read_u8(interconnect, value_location);
		match location {
			Location::Reg(reg) => self.write_reg_u8(reg, value),
			Location::Ram(address) => interconnect.write_u8(address, value)
		}
		self.pc += pc_offset;
		println!("executed: LD {:?}, {:?}", location, value_location);
	}
	fn read_reg_u16(&self, register: Register) -> u16 {
		match register {
			Register::BC => (self.reg_b as u16) << 8 | (self.reg_c as u16),
			Register::DE => (self.reg_b as u16) << 8 | (self.reg_c as u16),
			Register::HL => (self.reg_h as u16) << 8 | (self.reg_l as u16),
			Register::SP => self.sp,
			_ => panic!("Can't read u16 from register {:?}", register)
		}
	}
	fn read_u8(&self, interconnect: &Interconnect, location: Location) -> u8 {
		match location {
			Location::Reg(reg) => self.read_reg_u8(reg),
			Location::Ram(address) => interconnect.read_u8(address)
		}
	}
	fn read_reg_u8(&self, register: Register) -> u8 {
		match register {
			Register::A => self.reg_a,
			Register::B => self.reg_b,
			Register::C => self.reg_c,
			Register::D => self.reg_d,
			Register::E => self.reg_e,
			_ => panic!("Can't read u8 from register {:?}", register)
		}
	}
	fn write_reg_u16(&mut self, register: Register, value: u16){
		let upper = (value >> 8) as u8;
		let lower = (value & 0x00FF) as u8;
		match register {
			Register::BC => {
				self.reg_b = upper;
				self.reg_c = lower;
			},
			Register::DE => {
				self.reg_d = upper;
				self.reg_e = lower;
			},
			Register::HL => {
				self.reg_h = upper;
				self.reg_l = lower;
			},
			Register::SP => self.sp = value,
			_ => panic!("Can't write u16 {:#X} to register {:?}", value, register)
		}
	}
	fn write_reg_u8(&mut self, register: Register, value: u8){
		match register {
			Register::A => self.reg_a = value,
			Register::C => self.reg_c = value,
			_ => panic!("Can't write u8 {:#X} to register {:?}", value, register)
		}
	}
}
