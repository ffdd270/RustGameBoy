use std::os::raw::{c_uchar, c_ushort, c_short};

pub type Byte = c_uchar;
pub type SignedByte = char;
pub type Word = c_ushort;
pub type SignedWord = c_short;
