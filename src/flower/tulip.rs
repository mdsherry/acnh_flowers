use crate::genetics::constants::*;
use crate::genetics::*;

use flower_macros::flower_match;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tulip {
    genome: Genome3,
}

impl Tulip {
    pub fn colour(self) -> &'static str {
        flower_match!(
            White White White
            Yellow Yellow White
            Yellow Yellow Yellow
            Red Pink White
            Orange Yellow Yellow
            Orange Yellow Yellow
            Black Red Red
            Black Red Red
            Purple Purple Purple
            : self.genome
        )
    }

    pub fn white_from_seed() -> Self {
        Tulip { genome: R0Y0W1 }
    }

    pub fn red_from_seed() -> Self {
        Tulip { genome: R2Y0W1 }
    }

    pub fn yellow_from_seed() -> Self {
        Tulip { genome: R0Y2W0 }
    }
}

impl std::ops::Mul<Self> for Tulip {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self { genome: self.genome * other.genome }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tulip_colours() {
        assert_eq!("White", Tulip::white_from_seed().colour());
        assert_eq!("Red", Tulip::red_from_seed().colour());
        assert_eq!("Yellow", Tulip::yellow_from_seed().colour());
    }
}
