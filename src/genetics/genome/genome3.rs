use crate::genetics::allele::{RedA, WhiteA, YellowA};
use super::{Genome, Genome2};

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


impl std::ops::BitOr<WhiteA> for Genome2 {
    type Output = Genome3;
    fn bitor(self, other: WhiteA) -> Genome3 {
        gen23(self, other)
    }
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Genome3 {
    pub(super) code: u8,
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

pub struct GenomeIterator {
    idx: u8
}

impl Iterator for GenomeIterator {
    type Item=Genome3;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 27 {
            None
        } else {
            let rv = RedA::new(self.idx % 3) | YellowA::new((self.idx / 3) % 3) | WhiteA::new((self.idx / 9 ) % 3);
            self.idx += 1;
            Some(rv)
        }
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

    fn distinct_offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.red().distinct_offspring(other.red()).flat_map(move |r| {
            self.yellow().distinct_offspring(other.yellow()).flat_map(move |y| {
                self.white()
                    .distinct_offspring(other.white())
                    .map(move |w| r | y | w)
            })
        }))
    }

    fn all_genomes() -> Box<dyn Iterator<Item=Self>> {
        Box::new(GenomeIterator { idx: 0 })
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