use crate::genetics::constants::*;
use crate::genetics::*;
use flower_macros::flower_match;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cosmo {
    genome: Genome3,
}

impl Cosmo {
    pub fn colour(self) -> &'static str {
        flower_match! {
            White White White
            Yellow Yellow White
            Yellow Yellow Yellow
            Pink Pink Pink 
            Orange Orange Pink
            Orange Orange Orange
            Red Red Red
            Orange Orange Red
            Black Black Red
            : self.genome
        }
    }

    pub fn white_from_seed() -> Self {
        Cosmo { genome: R0Y0W1 }
    }

    pub fn red_from_seed() -> Self {
        Cosmo { genome: R2Y0W0 }
    }

    pub fn yellow_from_seed() -> Self {
        Cosmo { genome: R0Y2W1 }
    }
}

impl std::ops::Mul<Self> for Cosmo {
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
        assert_eq!("White", Cosmo::white_from_seed().colour());
        assert_eq!("Red", Cosmo::red_from_seed().colour());
        assert_eq!("Yellow", Cosmo::yellow_from_seed().colour());
    }
}
