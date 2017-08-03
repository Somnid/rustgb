use std::io::Write;
use std::io::Cursor;

#[derive(Debug)]
pub struct Interconnect {
	ram : Box<[u8]>,
	vram : Box<[u8]>
}

struct InterruptEnable{
	v_blank: bool,
	lcd_stat : bool,
	timer : bool,
	serial : bool,
	joypad : bool
}

struct InterruptFlag{
	v_blank: bool,
	lcd_stat : bool,
	timer : bool,
	serial : bool,
	joypad : bool
}

impl Interconnect {
	pub fn new(bootrom: &mut[u8]) -> Interconnect {
		let mut ram = vec![0u8; 8192];
		let mut ram_cursor = Cursor::new(ram);
		ram_cursor.write_all(&bootrom).unwrap();
		let ram_with_bootrom = ram_cursor.into_inner();

		Interconnect {
			ram : ram_with_bootrom.into_boxed_slice(),
			vram : vec![0u8; 8192].into_boxed_slice()
		}
	}
	pub fn read_u8(&self, address: u16) -> u8 {
		self.ram[address as usize]
	}
	pub fn read_u16(&self, address: u16) -> u16 {
		(self.read_u8(address + 1) as u16) << 8 | (self.read_u8(address) as u16)
	}
	pub fn write_u8(&mut self, address: u16, value: u8) {
		match address {
			0xFFFF => panic!("writing to unimplemented interrupt enable register"),
			0xFF80...0xFFFE => panic!("writing to unimplemented Zero Page"),
			0xFF00...0xFF7F => self.write_hardware_io_reg(value),
			0xFEA0...0xFEFF => panic!("Writing to unusable memory"),
			0xFE00...0xFE9F => panic!("Writing to OAM (Object Attribute Memory)"),
			0xE000...0xFDFF => panic!("Writing to Echo RAM (Do not use)"),
			0xD000...0xDFFF => panic!("Writing to interal RAM Bank 1-7"),
			0xC000...0xCFFF => panic!("Writing to interal RAM Bank 0"),
			0xA000...0xBFFF => panic!("Writing to Cartridge RAM"),
			0x8000...0x9FFF => self.write_video_ram_u8(address - 0x8000, value),
			0x4000...0x7FFF => panic!("Writing to Cartridge ROM, Bank 1-xx"),
			0x0150...0x3FFF => panic!("Writing to Cartridge ROM, Bank 0"),
			0x0100...0x014F => panic!("Writing to Cartridge Header"),
			0x0000...0x00FF => panic!("Writing to Restart and Interrupt Vectors"),
			_ => panic!("Writing to unimplemented part of memory. Address: {:#X}, Value: {:#X}", address, value)
		}
	}
	fn write_hardware_io_reg(&mut self, value: u8){
		println!("writing {:#X} to unimplemented Hardware I/O register", value);
	}
	fn write_video_ram_u8(&mut self, address: u16, value: u8){
		match address {
			0x1C00...0x1FFF => println!("Writing to BG map Data 2"),
			0x1800...0x1BFF => println!("Writing to BG Map Data 1"),
			0x0000...0x17FF => println!("Writing to character RAM"),
			_ => panic!("Writing to VRAM that should not exist.")
		}
		self.vram[address as usize] = value;
	}
	/*
	pub fn write_video_ram_u16(&mut self, address: u16, value: u16){
		println!("writing {:#X} to video RAM address: {:#X}", value, address);
		let upper = (value >> 8) as u8;
		let lower = (value & 0x00FF) as u8;
		self.vram[address as usize] = lower;
		self.vram[(address + 1) as usize] = upper;
	}*/
}
