use super::Flower;
use crate::genetics::constants::*;
use crate::genetics::*;
use flower_macros::flower_match;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pansy {
    genome: Genome3,
}

impl Flower for Pansy {
    type GenomeType = Genome3;

    fn colour(self) -> &'static str {
        flower_match! {
            White White Blue
            Yellow Yellow Blue
            Yellow Yellow Yellow

            Red Red Blue
            Orange Orange Orange
            Yellow Yellow Yellow

            Red Red Purple
            Red Red Purple
            Orange Orange Purple
            : self.genome
        }
    }

    fn name(self) -> &'static str {
        "pansy"
    }

    fn genome(self) -> Self::GenomeType {
        self.genome
    }

    fn from_genome(genome: Self::GenomeType) -> Self {
        Self { genome }
    }
}
impl Pansy {
    pub fn white_from_seed() -> Self {
        Pansy { genome: R0Y0W1 }
    }

    pub fn red_from_seed() -> Self {
        Pansy { genome: R2Y0W0 }
    }

    pub fn yellow_from_seed() -> Self {
        Pansy { genome: R0Y2W0 }
    }
}
impl std::fmt::Debug for Pansy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.debug(f)
    }
}

impl std::ops::Mul<Self> for Pansy {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            genome: self.genome * other.genome,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pansy_colours() {
        assert_eq!("White", Pansy::white_from_seed().colour());
        assert_eq!("Red", Pansy::red_from_seed().colour());
        assert_eq!("Yellow", Pansy::yellow_from_seed().colour());
    }
}
