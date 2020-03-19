mod system;

use crate::system::declare::type_def::{Byte, SignedByte, Word, SignedWord};
use crate::system::cartridge::{create_cartridge, Cartridge};

fn main()  {
    println!("Hello, world!" );

    let byte_temp : Byte = 255;
    let char_temp : SignedByte = 'c';
    let word_temp : Word = 20002;
    let signed_word_temp : SignedWord = 10001;

    println!("Byte? {}" ,byte_temp);
    println!("SignedByte {}", char_temp);
    println!("Word {}", word_temp);
    println!("SignedWord {}", signed_word_temp);

    let result_cartridge = create_cartridge();

    let cartridge = match result_cartridge // Rust의 Switch Case.
    {
        Ok(result_cartridge) => result_cartridge,
        Err(result_cartridge) => return
    };

    cartridge.load_memory();
}
