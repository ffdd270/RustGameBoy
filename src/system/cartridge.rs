use crate::system::declare::type_def::{Byte};
use std::fs::File;
use std::io;
use std::io::Read;

trait CartridgeTrait
{
    fn load_memory(&self);
}

struct Cartridge
{
    //memory : [Byte; 0x200000]
}


impl CartridgeTrait for Cartridge {
    fn load_memory( &self )
    {
    }
}


pub fn create_cartridge() -> io::Result<()>
{
    println!(" buffer.len()");
    let mut memory : [Byte; 0x200000] = [ 0; 0x200000 ];
    let mut file = File::open("rom/zelda.gb")?;

    let mut buffer : Vec<Byte> = Vec::new();
    file.read_to_end(&mut buffer)?;

    println!(" buffer.len() {}", buffer.len() );

    Ok(())
}