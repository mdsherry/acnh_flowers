pub mod constants;
use super::allele::{BlueA, RedA, WhiteA, YellowA};

pub trait Genome {
    fn offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>>;
}

pub const fn gen2(r: RedA, y: YellowA) -> Genome2 {
    Genome2 {
        code: y.mask() | r.mask(),
    }
}
pub const fn gen23(g: Genome2, w: WhiteA) -> Genome3 {
    Genome3 {
        code: w.mask() | g.code,
    }
}
pub const fn gen3(r: RedA, y: YellowA, w: WhiteA) -> Genome3 {
    Genome3 {
        code: w.mask() | y.mask() | r.mask(),
    }
}
pub const fn gen34(g: Genome3, b: BlueA) -> Genome4 {
    Genome4 {
        code: b.mask() | g.code,
    }
}
pub const fn gen4(r: RedA, y: YellowA, w: WhiteA, b: BlueA) -> Genome4 {
    Genome4 {
        code: b.mask() | w.mask() | y.mask() | r.mask(),
    }
}

impl std::ops::BitOr<YellowA> for RedA {
    type Output = Genome2;
    fn bitor(self, other: YellowA) -> Genome2 {
        gen2(self, other)
    }
}

impl std::ops::BitOr<WhiteA> for Genome2 {
    type Output = Genome3;
    fn bitor(self, other: WhiteA) -> Genome3 {
        gen23(self, other)
    }
}

impl std::ops::BitOr<BlueA> for Genome3 {
    type Output = Genome4;
    fn bitor(self, other: BlueA) -> Genome4 {
        gen34(self, other)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Genome2 {
    code: u8,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Genome3 {
    code: u8,
}

impl Genome3 {
    pub fn red(self) -> RedA {
        RedA::extract(self.code)
    }
    pub fn yellow(self) -> YellowA {
        YellowA::extract(self.code)
    }
    pub fn white(self) -> WhiteA {
        WhiteA::extract(self.code)
    }
}

impl Genome for Genome3 {
    fn offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.red().offspring(other.red()).flat_map(move |r| {
            self.yellow().offspring(other.yellow()).flat_map(move |y| {
                self.white()
                    .offspring(other.white())
                    .map(move |w| r | y | w)
            })
        }))
    }
}

impl std::fmt::Debug for Genome3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.red().fmt(f)?;
        self.yellow().fmt(f)?;
        self.white().fmt(f)
    }
}

impl std::ops::Mul<Genome3> for Genome3 {
    type Output = Genome3;

    fn mul(self, other: Genome3) -> Self::Output {
        (self.red() * other.red())
            | (self.yellow() * other.yellow())
            | (self.white() * other.white())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Genome4 {
    code: u8,
}

impl Genome4 {
    fn red(self) -> RedA {
        RedA::extract(self.code)
    }
    fn yellow(self) -> YellowA {
        YellowA::extract(self.code)
    }
    fn white(self) -> WhiteA {
        WhiteA::extract(self.code)
    }

    fn blue(self) -> BlueA {
        BlueA::extract(self.code)
    }
}

impl Genome for Genome4 {
    fn offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.red().offspring(other.red()).flat_map(move |r| {
            self.yellow().offspring(other.yellow()).flat_map(move |y| {
                self.white().offspring(other.white()).flat_map(move |w| {
                    self.blue()
                        .offspring(other.blue())
                        .map(move |b| r | y | w | b)
                })
            })
        }))
    }
}

impl std::ops::Mul<Genome4> for Genome4 {
    type Output = Genome4;

    fn mul(self, other: Genome4) -> Self::Output {
        (self.red() * other.red())
            | (self.yellow() * other.yellow())
            | (self.white() * other.white())
            | (self.blue() * other.blue())
    }
}

impl std::fmt::Debug for Genome4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.red().fmt(f)?;
        self.yellow().fmt(f)?;
        self.white().fmt(f)?;
        self.blue().fmt(f)
    }
}
