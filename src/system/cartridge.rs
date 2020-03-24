use crate::system::declare::type_def::{Byte};
use std::fs::File;
use std::io;
use std::io::Read;

pub struct Cartridge
{
    memory_vec : Vec<Byte>,
    filename : String
}


impl Cartridge {
    pub fn load_memory( self )
    {
        println!("Memory Size : {} ", self.memory_vec.len());
        println!("Filename : {}", self.filename );
    }
}

pub fn create_cartridge() -> io::Result<Cartridge>
{
    let mut file = File::open("rom/zelda.gb")?;
    let mut buffer : Vec<Byte> = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok( Cartridge{ memory_vec: buffer, filename: "rom/zelda.gb".to_string() } )
}