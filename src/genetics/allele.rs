use super::{Blue, Gene, Red, White, Yellow};
use rand::prelude::*;
use std::marker::PhantomData;
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Allele<G: Gene>(u8, PhantomData<G>);

// impl<G: Gene> Allele<G> {
//     pub const fn extract(code: u8) -> Self {
//         Allele((code >> G::OFFSET) & 0b11, PhantomData)
//     }

//     pub const fn mask(self) -> u8 {
//         self.0 << G::OFFSET
//     }
// }
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
    pub fn new(code: u8) -> Self {
        assert!(code == 0 || code == 1 || code == 2);
        Allele(code, PhantomData)
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
