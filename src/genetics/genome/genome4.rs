use crate::genetics::allele::{BlueA, RedA, WhiteA, YellowA};
use super::{Genome, Genome3};

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


impl std::ops::BitOr<BlueA> for Genome3 {
    type Output = Genome4;
    fn bitor(self, other: BlueA) -> Genome4 {
        gen34(self, other)
    }
}



#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Genome4 {
    code: u8,
}

impl Genome4 {
    pub fn red(self) -> RedA {
        RedA::extract(self.code)
    }
    pub fn yellow(self) -> YellowA {
        YellowA::extract(self.code)
    }
    pub fn white(self) -> WhiteA {
        WhiteA::extract(self.code)
    }

    pub fn blue(self) -> BlueA {
        BlueA::extract(self.code)
    }
}


pub struct GenomeIterator {
    idx: u8
}

impl Iterator for GenomeIterator {
    type Item=Genome4;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 81 {
            None
        } else {
            let rv = RedA::new(self.idx % 3) | YellowA::new((self.idx / 3) % 3) | WhiteA::new((self.idx / 9 ) % 3) | BlueA::new((self.idx / 27) % 3);
            self.idx += 1;
            Some(rv)
        }
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
    fn distinct_offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.red().distinct_offspring(other.red()).flat_map(move |r| {
            self.yellow().distinct_offspring(other.yellow()).flat_map(move |y| {
                self.white().distinct_offspring(other.white()).flat_map(move |w| {
                    self.blue()
                        .distinct_offspring(other.blue())
                        .map(move |b| r | y | w | b)
                })
            })
        }))
    }
    fn all_genomes() -> Box<dyn Iterator<Item=Self>> {
        Box::new(GenomeIterator { idx: 0 })
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
