pub mod constants;
pub mod genome2;
pub mod genome3;
pub mod genome4;

pub use genome2::{Genome2, gen2};
pub use genome3::{Genome3, gen23, gen3};
pub use genome4::{Genome4, gen34, gen4};

pub trait Genome {
    fn offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>>;
    fn distinct_offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>>;
    fn all_genomes() -> Box<dyn Iterator<Item=Self>>;
}
