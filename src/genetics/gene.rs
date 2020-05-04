//! Genes track metadata relating to the position of an allele within a genome,
//! and its debug representation.
//! There are only four genes: Red, Yellow, White and Blue.
pub trait Gene: Copy {
    const OFFSET: u8;
    const DISPLAY: [&'static str; 3];
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Red;
impl Gene for Red {
    const OFFSET: u8 = 0;
    const DISPLAY: [&'static str; 3] = ["rr", "Rr", "RR"];
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Yellow;
impl Gene for Yellow {
    const OFFSET: u8 = 2;
    const DISPLAY: [&'static str; 3] = ["yy", "Yy", "YY"];
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct White;
impl Gene for White {
    const OFFSET: u8 = 4;
    const DISPLAY: [&'static str; 3] = ["ww", "Ww", "WW"];
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Blue;
impl Gene for Blue {
    const OFFSET: u8 = 6;
    const DISPLAY: [&'static str; 3] = ["bb", "Bb", "BB"];
}
