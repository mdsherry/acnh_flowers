use crate::genetics::constants::*;
use crate::genetics::*;

use flower_macros::flower_match;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Windflower {
    genome: Genome3,
}

impl Windflower {
    pub fn colour(self) -> &'static str {
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
        Self { genome: self.genome * other.genome }
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
