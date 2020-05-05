//! The various flowers of Animal Crossing: New Horizons
mod colour;
pub use colour::Colour;

mod pansy;
pub use pansy::Pansy;
mod tulip;
pub use tulip::Tulip;
mod rose;
pub use rose::Rose;
mod cosmo;
pub use cosmo::Cosmo;
mod lily;
pub use lily::Lily;
mod hyacinth;
pub use hyacinth::Hyacinth;
mod windflower;
pub use windflower::Windflower;
mod mum;
pub use mum::Mum;

use crate::genetics::{Genome, Genome3};

pub trait Flower: Sized + 'static + Copy {
    type GenomeType: Genome + std::fmt::Debug;

    fn colour(self) -> Colour;
    fn name(self) -> &'static str;

    fn genome(self) -> Self::GenomeType;

    fn offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.genome()
                .offspring(other.genome())
                .map(Self::from_genome),
        )
    }

    fn distinct_offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.genome()
                .distinct_offspring(other.genome())
                .map(Self::from_genome),
        )
    }

    fn distinguishable_offspring(self, other: Self) -> Box<dyn Iterator<Item=Self>> {
        let mut offspring: Vec<_> = self.distinct_offspring(other).collect();
        offspring.sort_unstable_by_key(|f| f.colour());
        let mut idx = 0;
        Box::new(std::iter::from_fn(move || {
            while idx < offspring.len() {
                let colour = offspring[idx].colour();
                let mut next = idx + 1;
                while next < offspring.len() && offspring[next].colour() == colour {
                    next += 1;
                }
                let old_idx = idx;
                idx = next;
                if next == old_idx + 1 {
                    return Some(offspring[old_idx])
                }
            }
            None
        }))
    }

    fn from_genome(genome: Self::GenomeType) -> Self;

    fn debug(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        self.colour().fmt(f)?;
        f.write_str(" ")?;
        f.write_str(self.name())?;
        f.write_str(" (")?;
        self.genome().fmt(f)?;
        f.write_str(")")
    }

    fn all_seeds() -> &'static [Self];
    fn all_wild() -> &'static [Self];
}
