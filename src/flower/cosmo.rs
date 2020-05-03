use crate::genetics::constants::*;
use crate::genetics::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cosmo {
    genome: Genome3,
}

impl Cosmo {
    pub fn colour(self) -> &'static str {
        match self.genome {
            R0Y0W0 | R0Y0W1 | R0Y0W2 | R0Y1W2 => "White",
            R0Y1W0 | R0Y1W1 | R0Y2W0 | R0Y2W1 | R0Y2W2 => "Yellow",
            R1Y0W0 | R1Y0W1 | R1Y0W2 | R1Y1W2 => "Pink",
            R1Y1W0 | R1Y1W1 | R1Y2W0 | R1Y2W1 | R1Y2W2 | R2Y1W0 | R2Y1W1 => "Orange",
            R2Y0W0 | R2Y0W1 | R2Y0W2 | R2Y1W2 | R2Y2W2 => "Red",
            R2Y2W0 | R2Y2W1 => "Black",
            _ => panic!("No colour for {:?}", self.genome),
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_colours() {
        assert_eq!("White", Cosmo::white_from_seed().colour());
        assert_eq!("Red", Cosmo::red_from_seed().colour());
        assert_eq!("Yellow", Cosmo::yellow_from_seed().colour());
    }

    const EXPECTED_TABLE: &str = "\
WWW
YYW
YYY
PPP
OOP
OOO
RRR
OOR
KKR
";
    fn make_table() -> String {
        let mut rv = String::with_capacity(100);
        for r in 0..=2 {
            for y in 0..=2 {
                for w in 0..=2 {
                    let genome = RedA::new(r) | YellowA::new(y) | WhiteA::new(w);
                    let colour = Cosmo { genome }.colour();
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
