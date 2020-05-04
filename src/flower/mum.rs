use super::Flower;
use crate::genetics::constants::*;
use crate::genetics::*;
use flower_macros::flower_match;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mum {
    genome: Genome3,
}

impl Flower for Mum {
    type GenomeType = Genome3;

    fn colour(self) -> &'static str {
        flower_match! {
            White White Purple
            Yellow Yellow White
            Yellow Yellow Yellow

            Pink Pink Pink
            Yellow Red Pink
            Purple Purple Purple

            Red Red Red
            Purple Purple Red
            Green Green Red
            : self.genome
        }
    }

    fn name(self) -> &'static str {
        "mum"
    }

    fn genome(self) -> Self::GenomeType {
        self.genome
    }

    fn from_genome(genome: Self::GenomeType) -> Self {
        Self { genome }
    }
}
impl Mum {
    pub fn white_from_seed() -> Self {
        Mum { genome: R0Y0W1 }
    }

    pub fn red_from_seed() -> Self {
        Mum { genome: R2Y0W0 }
    }

    pub fn yellow_from_seed() -> Self {
        Mum { genome: R0Y2W0 }
    }
}

impl std::ops::Mul<Self> for Mum {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            genome: self.genome * other.genome,
        }
    }
}
impl std::fmt::Debug for Mum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.debug(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_colours() {
        assert_eq!("White", Mum::white_from_seed().colour());
        assert_eq!("Red", Mum::red_from_seed().colour());
        assert_eq!("Yellow", Mum::yellow_from_seed().colour());
    }
}
