use crate::genetics::allele::{RedA, YellowA};

pub const fn gen2(r: RedA, y: YellowA) -> Genome2 {
    Genome2 {
        code: y.mask() | r.mask(),
    }
}



impl std::ops::BitOr<YellowA> for RedA {
    type Output = Genome2;
    fn bitor(self, other: YellowA) -> Genome2 {
        gen2(self, other)
    }
}



#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Genome2 {
    pub(super) code: u8,
}