mod allele;
mod gene;
mod genome;

pub use allele::*;
pub use gene::*;
pub use genome::*;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_breeding_allele() {
        assert_eq!(R0, R0 * R0);
        assert_eq!(R2, R2 * R2);
        assert_eq!(R1, R0 * R2);
        assert_eq!(R1, R2 * R0);
    }

    #[test]
    fn test_debug_output() {
        assert_eq!("rrYyWW", &format!("{:?}", R0 | Y1 | W2));
        assert_eq!("rrYyWWBb", &format!("{:?}", R0 | Y1 | W2 | B1));
    }
}
