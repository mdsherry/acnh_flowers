use super::Flower;
use crate::genetics::constants::*;
use crate::genetics::*;
use flower_macros::flower_match;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Windflower {
    genome: Genome3,
}

impl Flower for Windflower {
    type GenomeType = Genome3;

    fn colour(self) -> &'static str {
        flower_match! {
            White White Blue
            Orange Orange Blue
            Orange Orange Orange

            Red Red Blue
            Pink Pink Pink
            Orange Orange Orange

            Red Red Purple
            Red Red Purple
            Pink Pink Purple
            : self.genome
        }
    }

    fn name(self) -> &'static str {
        "windflower"
    }

    fn genome(self) -> Self::GenomeType {
        self.genome
    }

    fn from_genome(genome: Self::GenomeType) -> Self {
        Self { genome }
    }
}
impl Windflower {
    pub fn white_from_seed() -> Self {
        Windflower { genome: R0Y0W1 }
    }

    pub fn red_from_seed() -> Self {
        Windflower { genome: R2Y0W0 }
    }

    pub fn orange_from_seed() -> Self {
        Windflower { genome: R0Y2W0 }
    }
}

impl std::ops::Mul<Self> for Windflower {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            genome: self.genome * other.genome,
        }
    }
}
impl std::fmt::Debug for Windflower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.debug(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_colours() {
        assert_eq!("White", Windflower::white_from_seed().colour());
        assert_eq!("Red", Windflower::red_from_seed().colour());
        assert_eq!("Orange", Windflower::orange_from_seed().colour());
    }
}
