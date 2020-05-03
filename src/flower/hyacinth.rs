use crate::genetics::constants::*;
use crate::genetics::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hyacinth {
    genome: Genome3,
}

impl Hyacinth {
    pub fn colour(self) -> &'static str {
        match self.genome {
            R0Y0W0 | R0Y0W1 | R0Y1W2 | R1Y0W2 => "White",
            R0Y1W0 | R0Y1W1 | R0Y2W0 | R0Y2W1 | R0Y2W2 | R1Y1W1 | R1Y1W2 | R1Y2W1 | R1Y2W2 => {
                "Yellow"
            }
            R1Y0W0 | R2Y0W0 | R2Y0W1 | R2Y0W2 | R2Y1W2 => "Red",
            R1Y1W0 | R1Y2W0 => "Orange",
            R1Y0W1 => "Pink",
            R2Y2W0 | R2Y2W1 | R2Y2W2 => "Purple",
            R0Y0W2 | R2Y1W0 | R2Y1W1 => "Blue",
            _ => "Unknown",
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_colours() {
        assert_eq!("White", Hyacinth::white_from_seed().colour());
        assert_eq!("Red", Hyacinth::red_from_seed().colour());
        assert_eq!("Yellow", Hyacinth::yellow_from_seed().colour());
    }

    const EXPECTED_TABLE: &str = "\
WWB
YYW
YYY
RPW
OYY
OYY
RRR
BBR
UUU
";
    fn make_table() -> String {
        let mut rv = String::with_capacity(100);
        for r in 0..=2 {
            for y in 0..=2 {
                for w in 0..=2 {
                    let genome = RedA::new(r) | YellowA::new(y) | WhiteA::new(w);
                    let colour = Hyacinth { genome }.colour();
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

    // #[test]
    #[allow(dead_code)]
    fn create_matcher() {
        crate::flower::test::build_matcher3(EXPECTED_TABLE);
    }
}
