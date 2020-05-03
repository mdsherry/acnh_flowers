use crate::genetics::constants::*;
use crate::genetics::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tulip {
    genome: Genome3,
}

impl Tulip {
    pub fn colour(self) -> &'static str {
        match self.genome {
            R0Y0W0 | R0Y0W1 | R0Y0W2 | R0Y1W2 | R1Y0W2 => "White",
            R0Y1W0 | R0Y1W1 | R0Y2W0 | R0Y2W1 | R0Y2W2 | R1Y1W1 | R1Y1W2 | R1Y2W1 | R1Y2W2 => {
                "Yellow"
            }
            R1Y0W0 | R2Y0W1 | R2Y0W2 | R2Y1W1 | R2Y1W2 => "Red",
            R1Y0W1 => "Pink",
            R1Y1W0 | R1Y2W0 => "Orange",
            R2Y0W0 | R2Y1W0 => "Black",
            R2Y2W0 | R2Y2W1 | R2Y2W2 => "Purple",
            _ => unreachable!("No colour for {:?}", self.genome),
        }
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_tulip_colours() {
        assert_eq!("White", Tulip::white_from_seed().colour());
        assert_eq!("Red", Tulip::red_from_seed().colour());
        assert_eq!("Yellow", Tulip::yellow_from_seed().colour());
    }

    const EXPECTED_TABLE: &str = "\
WWW
YYW
YYY
RPW
OYY
OYY
KRR
KRR
UUU
";
    fn make_table() -> String {
        let mut rv = String::with_capacity(100);
        for r in 0..=2 {
            for y in 0..=2 {
                for w in 0..=2 {
                    let genome = RedA::new(r) | YellowA::new(y) | WhiteA::new(w);
                    let colour = Tulip { genome }.colour();
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
        assert_eq!(EXPECTED_TABLE, &make_table());
    }
}
