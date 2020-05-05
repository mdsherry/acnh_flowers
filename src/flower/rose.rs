use super::Flower;
use crate::genetics::constants::*;
use crate::genetics::*;
use flower_macros::flower_match4;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Rose {
    genome: Genome4,
}

impl Flower for Rose {
    type GenomeType = Genome4;

    fn colour(self) -> super::Colour {
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

    fn name(self) -> &'static str {
        "rose"
    }

    fn genome(self) -> Self::GenomeType {
        self.genome
    }

    fn from_genome(genome: Self::GenomeType) -> Self {
        Self { genome }
    }

    fn all_seeds() -> &'static [Rose] {
        ALL_SEEDS
    }
    fn all_wild() -> &'static [Rose] {
        ALL_WILD
    }
}

impl Rose {
    pub const fn white_from_seed() -> Self {
        Rose { genome: R0Y0W1B0 }
    }

    pub const fn red_from_seed() -> Self {
        Rose { genome: R2Y0W0B1 }
    }

    pub const fn yellow_from_seed() -> Self {
        Rose { genome: R0Y1W0B0 }
    }

    pub const fn wild_pink() -> Self {
        Rose { genome: R2Y0W2B2 }
    }
    
    pub const fn wild_orange() -> Self {
        Rose { genome: R2Y2W1B1 }
    }
}

static ALL_SEEDS: &'static [Rose] = &[
    Rose::white_from_seed(), Rose::red_from_seed(), Rose::yellow_from_seed()
];
static ALL_WILD: &'static [Rose] = &[
    Rose::wild_pink(), Rose::wild_orange()
];

impl std::ops::Mul<Self> for Rose {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            genome: self.genome * other.genome,
        }
    }
}
impl std::fmt::Debug for Rose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.debug(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::flower::Colour;
    #[test]
    fn test_rose_colours() {
        assert_eq!(Colour::White, Rose::white_from_seed().colour());
        assert_eq!(Colour::Red, Rose::red_from_seed().colour());
        assert_eq!(Colour::Yellow, Rose::yellow_from_seed().colour());
    }
}
