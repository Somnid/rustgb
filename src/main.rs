use std::env;
mod file_helper;
mod gameboy;
mod cpu;
mod interconnect;
mod add_signed;
mod debugger;

fn main() {
    match env::args().nth(1){
        Some(boot_rom_path) => {
            match file_helper::read_file_bytes(boot_rom_path){
                Ok(mut boot_rom) => {
                    let mut gameboy = gameboy::Gameboy::new(boot_rom.as_mut_slice());
                    let mut debugger = debugger::Debugger::new(&mut gameboy);
					debugger.run();
                }
                Err(message) => panic!("{}", message)
            }
        }
        None => println!("No bootloader path supplied.")
    }
}
