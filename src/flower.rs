//! The various flowers of Animal Crossing: New Horizons

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

    fn colour(self) -> &'static str;
    fn name(self) -> &'static str;

    fn genome(self) -> Self::GenomeType;

    fn offspring(self, other: Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.genome()
                .offspring(other.genome())
                .map(Self::from_genome),
        )
    }

    fn from_genome(genome: Self::GenomeType) -> Self;

    fn debug(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        f.write_str(self.colour())?;
        f.write_str(" ")?;
        f.write_str(self.name())?;
        f.write_str(" (")?;
        self.genome().fmt(f)?;
        f.write_str(")")
    }
}
