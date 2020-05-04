//! An allele represnts a specifc instance of a gene within a genome.
//! It has three possible values: both-recessive (xx), hybrid (Xx) and both dominant (XX).
//!
//! Two alleles can be bred together by multiplying them.

use super::{Blue, Gene, Red, White, Yellow};
use rand::prelude::*;
use std::marker::PhantomData;
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Allele<G>(u8, PhantomData<G>);

// Implement the alleles via macro to allow the use of const fn
macro_rules! implAllele {
    ($gene:ident) => {
        impl Allele<$gene> {
            pub const fn extract(code: u8) -> Self {
                Allele((code >> $gene::OFFSET) & 0b11, PhantomData)
            }

            pub const fn mask(self) -> u8 {
                self.0 << $gene::OFFSET
            }
        }
    };
}

implAllele!(Red);
implAllele!(Yellow);
implAllele!(White);
implAllele!(Blue);

impl<G: Gene> Allele<G> {
    /// Code can be one of 0 (xx), 1 (Xx) or 2 (XX)
    pub fn new(code: u8) -> Self {
        assert!(code == 0 || code == 1 || code == 2);
        Allele(code, PhantomData)
    }

    pub fn offspring(self, other: Allele<G>) -> AlleleIterator<G> {
        AlleleIterator {
            left: self,
            right: other,
            idx: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AlleleIterator<G: Gene> {
    left: Allele<G>,
    right: Allele<G>,
    idx: u8,
}

impl<G: Gene> Iterator for AlleleIterator<G> {
    type Item = Allele<G>;
    fn next(&mut self) -> Option<Self::Item> {
        let rv = match (self.left.0, self.right.0) {
            (0, 0) => {
                if self.idx == 0 {
                    Some(0)
                } else {
                    None
                }
            }
            (0, 2) | (2, 0) => {
                if self.idx == 0 {
                    Some(1)
                } else {
                    None
                }
            }
            (2, 2) => {
                if self.idx == 0 {
                    Some(2)
                } else {
                    None
                }
            }
            (0, 1) | (1, 0) => match self.idx {
                0 => Some(0),
                1 => Some(1),
                _ => None,
            },
            (1, 1) => match self.idx {
                0 => Some(0),
                1 => Some(1),
                2 => Some(1),
                3 => Some(2),
                _ => None,
            },
            (2, 1) | (1, 2) => match self.idx {
                0 => Some(1),
                1 => Some(2),
                _ => None,
            },
            _ => unreachable!(),
        };
        self.idx += 1;
        let mut template = self.left;
        rv.map(|v| {
            template.0 = v;
            template
        })
    }
}

impl<G: Gene> std::ops::Mul<Allele<G>> for Allele<G> {
    type Output = Allele<G>;

    fn mul(self, other: Allele<G>) -> Self::Output {
        let mut rng = rand::thread_rng();
        Allele::<G>(
            match (self.0, other.0) {
                (0, 0) => 0,
                (0, 2) | (2, 0) => 1,
                (2, 2) => 2,
                (0, 1) | (1, 0) => *[0, 1].choose(&mut rng).expect("Always unempty"),
                (1, 1) => *[0, 1, 1, 2].choose(&mut rng).expect("Always unempty"),
                (2, 1) | (1, 2) => *[1, 2].choose(&mut rng).expect("Always unempty"),
                _ => unreachable!(),
            },
            PhantomData,
        )
    }
}

impl<G: Gene> std::fmt::Debug for Allele<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(G::DISPLAY[self.0 as usize])
    }
}

pub type RedA = Allele<Red>;
pub type YellowA = Allele<Yellow>;
pub type WhiteA = Allele<White>;
pub type BlueA = Allele<Blue>;

pub const R0: RedA = Allele::<Red>(0, PhantomData);
pub const R1: RedA = Allele::<Red>(1, PhantomData);
pub const R2: RedA = Allele::<Red>(2, PhantomData);

pub const Y0: YellowA = Allele::<Yellow>(0, PhantomData);
pub const Y1: YellowA = Allele::<Yellow>(1, PhantomData);
pub const Y2: YellowA = Allele::<Yellow>(2, PhantomData);

pub const W0: WhiteA = Allele::<White>(0, PhantomData);
pub const W1: WhiteA = Allele::<White>(1, PhantomData);
pub const W2: WhiteA = Allele::<White>(2, PhantomData);

pub const B0: BlueA = Allele::<Blue>(0, PhantomData);
pub const B1: BlueA = Allele::<Blue>(1, PhantomData);
pub const B2: BlueA = Allele::<Blue>(2, PhantomData);
