pub trait Gene {
    const OFFSET: u8;
    const DISPLAY: [&'static str; 3];
}

#[derive(Eq, PartialEq)]
pub struct Red;
impl Gene for Red {
    const OFFSET: u8 = 0;
    const DISPLAY: [&'static str; 3] = ["rr", "Rr", "RR"];
}

#[derive(Eq, PartialEq)]
pub struct Yellow;
impl Gene for Yellow {
    const OFFSET: u8 = 2;
    const DISPLAY: [&'static str; 3] = ["yy", "Yy", "YY"];
}

#[derive(Eq, PartialEq)]
pub struct White;
impl Gene for White {
    const OFFSET: u8 = 4;
    const DISPLAY: [&'static str; 3] = ["ww", "Ww", "WW"];
}

#[derive(Eq, PartialEq)]
pub struct Blue;
impl Gene for Blue {
    const OFFSET: u8 = 6;
    const DISPLAY: [&'static str; 3] = ["bb", "Bb", "BB"];
}
