use crate::genetics::constants::*;
use crate::genetics::*;
use flower_macros::flower_match;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hyacinth {
    genome: Genome3,
}

impl Hyacinth {
    pub fn colour(self) -> &'static str {
        flower_match! {
            White White Blue
            Yellow Yellow White
            Yellow Yellow Yellow

            Red Pink White
            Orange Yellow Yellow
            Orange Yellow Yellow

            Red Red Red
            Blue Blue Red
            Purple Purple Purple
            : self.genome
        }
    }

    pub fn white_from_seed() -> Self {
        Hyacinth { genome: R0Y0W1 }
    }

    pub fn red_from_seed() -> Self {
        Hyacinth { genome: R2Y0W1 }
    }

    pub fn yellow_from_seed() -> Self {
        Hyacinth { genome: R0Y2W0 }
    }
}

impl std::ops::Mul<Self> for Hyacinth {
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
        assert_eq!("White", Hyacinth::white_from_seed().colour());
        assert_eq!("Red", Hyacinth::red_from_seed().colour());
        assert_eq!("Yellow", Hyacinth::yellow_from_seed().colour());
    }
}
