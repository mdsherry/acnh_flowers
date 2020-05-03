use crate::genetics::constants::*;
use crate::genetics::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rose {
    genome: Genome4,
}

impl Rose {
    pub fn colour(self) -> &'static str {
        match self.genome {
            R0Y0W0B0 | R0Y0W0B1 | R0Y0W0B2 | R0Y0W1B0 | R0Y0W1B1 | R0Y0W1B2 | R0Y1W1B0
            | R0Y1W1B1 | R0Y1W1B2 | R0Y2W2B0 | R0Y2W2B1 | R0Y2W2B2 | R1Y0W0B2 | R1Y0W1B2
            | R1Y1W1B2 | R1Y2W2B2 | R2Y1W1B2 | R2Y2W2B2 => "White",
            R0Y0W2B0 | R0Y0W2B1 | R0Y0W2B2 | R0Y1W2B0 | R0Y1W2B1 | R0Y1W2B2 | R1Y0W2B2
            | R1Y1W2B2 | R2Y1W2B2 => "Purple",
            R0Y1W0B0 | R0Y1W0B1 | R0Y1W0B2 | R0Y2W0B0 | R0Y2W0B1 | R0Y2W0B2 | R0Y2W1B0
            | R0Y2W1B1 | R0Y2W1B2 | R1Y1W0B1 | R1Y1W0B2 | R1Y2W0B1 | R1Y2W0B2 | R1Y2W1B1
            | R1Y2W1B2 | R2Y1W0B2 | R2Y2W0B2 | R2Y2W1B2 => "Yellow",
            R1Y0W0B0 | R1Y0W1B0 | R1Y0W2B0 | R1Y1W1B0 | R1Y1W2B0 | R1Y2W2B0 | R2Y0W0B1
            | R2Y0W1B1 | R2Y0W2B1 | R2Y1W1B0 | R2Y1W1B1 | R2Y1W2B1 | R2Y2W2B1 => "Red",
            R1Y1W0B0 | R1Y2W0B0 | R1Y2W1B0 | R2Y1W0B0 | R2Y1W0B1 | R2Y2W0B0 | R2Y2W0B1
            | R2Y2W1B0 | R2Y2W1B1 => "Orange",
            R1Y0W0B1 | R1Y0W1B1 | R1Y0W2B1 | R1Y1W1B1 | R1Y1W2B1 | R1Y2W2B1 | R2Y0W0B2
            | R2Y0W1B2 | R2Y0W2B2 => "Pink",
            R2Y0W0B0 | R2Y0W1B0 | R2Y0W2B0 | R2Y1W2B0 => "Black",
            R2Y2W2B0 => "Blue",
            _ => unreachable!("No colour for {:?}", self.genome),
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rose_colours() {
        assert_eq!("White", Rose::white_from_seed().colour());
        assert_eq!("Red", Rose::red_from_seed().colour());
        assert_eq!("Yellow", Rose::yellow_from_seed().colour());
    }

    #[test]
    fn test_table() {
        crate::flower::test::compare_tables(EXPECTED_TABLE, &make_table());
    }

    const EXPECTED_TABLE: &str = "\
WWWWWWUUU
YYYWWWUUU
YYYYYYWWW
RPWRPWRPU
OYYRPWRPU
OYYOYYRPW
KRPKRPKRP
OOYRRWKRU
OOYOOYBRW
";

    fn make_table() -> String {
        let mut rv = String::with_capacity(100);
        for r in 0..=2 {
            for y in 0..=2 {
                for w in 0..=2 {
                    for b in 0..=2 {
                        let genome =
                            RedA::new(r) | YellowA::new(y) | WhiteA::new(w) | BlueA::new(b);
                        let colour = Rose { genome }.colour();
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
                }
                rv.push('\n');
            }
        }
        rv
    }
}
