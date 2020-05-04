use super::Flower;
use crate::genetics::constants::*;
use crate::genetics::*;
use flower_macros::flower_match;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lily {
    genome: Genome3,
}

impl Flower for Lily {
    type GenomeType = Genome3;

    fn colour(self) -> &'static str {
        flower_match! {
            White White White
            Yellow White White
            Yellow Yellow White

            Red Pink White
            Orange Yellow Yellow
            Orange Yellow Yellow

            Black Red Pink
            Black Red Pink
            Orange Orange White
            : self.genome
        }
    }

    fn name(self) -> &'static str {
        "lily"
    }

    fn genome(self) -> Self::GenomeType {
        self.genome
    }

    fn from_genome(genome: Self::GenomeType) -> Self {
        Self { genome }
    }
}

impl Lily {
    pub fn white_from_seed() -> Self {
        Lily { genome: R0Y0W2 }
    }

    pub fn red_from_seed() -> Self {
        Lily { genome: R2Y0W1 }
    }

    pub fn yellow_from_seed() -> Self {
        Lily { genome: R0Y2W0 }
    }
}

impl std::ops::Mul<Self> for Lily {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            genome: self.genome * other.genome,
        }
    }
}
impl std::fmt::Debug for Lily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.debug(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_colours() {
        assert_eq!("White", Lily::white_from_seed().colour());
        assert_eq!("Red", Lily::red_from_seed().colour());
        assert_eq!("Yellow", Lily::yellow_from_seed().colour());
    }
}
