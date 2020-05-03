use crate::genetics::constants::*;
use crate::genetics::*;

use flower_macros::flower_match4;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rose {
    genome: Genome4,
}

impl Rose {
    pub fn colour(self) -> &'static str {
        flower_match4! {
            White White White
            White White White
            Purple Purple Purple
            
            Yellow Yellow Yellow
            White White White
            Purple Purple Purple

            Yellow Yellow Yellow
            Yellow Yellow Yellow
            White White White
            

            Red Pink White
            Red Pink White
            Red Pink Purple

            Orange Yellow Yellow
            Red Pink White
            Red Pink Purple

            Orange Yellow Yellow
            Orange Yellow Yellow
            Red Pink White
            

            Black Red Pink
            Black Red Pink
            Black Red Pink

            Orange Orange Yellow
            Red Red White
            Black Red Purple

            Orange Orange Yellow
            Orange Orange Yellow
            Blue Red White
            : self.genome
        }
    }

    pub fn white_from_seed() -> Self {
        Rose { genome: R0Y0W1B0 }
    }

    pub fn red_from_seed() -> Self {
        Rose { genome: R2Y0W0B1 }
    }

    pub fn yellow_from_seed() -> Self {
        Rose { genome: R0Y1W0B0 }
    }
}

impl std::ops::Mul<Self> for Rose {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self { genome: self.genome * other.genome }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rose_colours() {
        assert_eq!("White", Rose::white_from_seed().colour());
        assert_eq!("Red", Rose::red_from_seed().colour());
        assert_eq!("Yellow", Rose::yellow_from_seed().colour());
    }

}
