use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Colour {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    White,
    Black,
}

use Colour::*;

impl Colour {
    pub fn name(self) -> &'static str {
        match self {
            Red => "Red",
            Orange => "Orange",
            Yellow => "Yellow",
            Green => "Green",
            Blue => "Blue",
            Purple => "Purple",
            Pink => "Pink",
            White => "White",
            Black => "Black",            
        }
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}