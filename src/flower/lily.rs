use crate::genetics::constants::*;
use crate::genetics::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lily {
    genome: Genome3,
}

impl Lily {
    pub fn colour(self) -> &'static str {
        match self.genome {
            R0Y0W0 | R0Y0W1 | R0Y0W2 | R0Y1W1 | R0Y1W2 | R0Y2W2 | R1Y0W2 | R2Y2W2 => "White",
            R0Y1W0 | R0Y2W0 | R0Y2W1 | R1Y1W1 | R1Y1W2 | R1Y2W1 | R1Y2W2 => "Yellow",
            R1Y0W0 | R2Y0W1 | R2Y1W1 => "Red",
            R1Y0W1 | R2Y0W2 | R2Y1W2 => "Pink",
            R1Y1W0 | R1Y2W0 | R2Y2W0 | R2Y2W1 => "Orange",
            R2Y0W0 | R2Y1W0 => "Black",
            _ => "Unknown",
        }
    }

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_colours() {
        assert_eq!("White", Lily::white_from_seed().colour());
        assert_eq!("Red", Lily::red_from_seed().colour());
        assert_eq!("Yellow", Lily::yellow_from_seed().colour());
    }

    const EXPECTED_TABLE: &str = "\
WWW
YWW
YYW
RPW
OYY
OYY
KRP
KRP
OOW
";
    fn make_table() -> String {
        let mut rv = String::with_capacity(100);
        for r in 0..=2 {
            for y in 0..=2 {
                for w in 0..=2 {
                    let genome = RedA::new(r) | YellowA::new(y) | WhiteA::new(w);
                    let colour = Lily { genome }.colour();
                    let letter = match colour {
                        "Red" => 'R',
                        "White" => 'W',
                        "Pink" => 'P',
                        "Blue" => 'B',
                        "Black" => 'K',
                        "Yellow" => 'Y',
                        "Orange" => 'O',
                        "Purple" => 'U',
                        "Unknown" => '.',
                        _ => unreachable!("Unrecognized colour {}", colour),
                    };
                    rv.push(letter);
                }
                rv.push('\n');
            }
        }
        rv
    }
    #[test]
    fn test_table() {
        crate::flower::test::compare_tables(EXPECTED_TABLE, &make_table());
    }
}
