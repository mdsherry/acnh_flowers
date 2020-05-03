use super::{gen3, gen4, Genome3, Genome4};
use crate::genetics::allele::{B0, B1, B2, R0, R1, R2, W0, W1, W2, Y0, Y1, Y2};

// Genome 3 patterns

pub const R0Y0W0: Genome3 = gen3(R0, Y0, W0);
pub const R0Y0W1: Genome3 = gen3(R0, Y0, W1);
pub const R0Y0W2: Genome3 = gen3(R0, Y0, W2);

pub const R0Y1W0: Genome3 = gen3(R0, Y1, W0);
pub const R0Y1W1: Genome3 = gen3(R0, Y1, W1);
pub const R0Y1W2: Genome3 = gen3(R0, Y1, W2);

pub const R0Y2W0: Genome3 = gen3(R0, Y2, W0);
pub const R0Y2W1: Genome3 = gen3(R0, Y2, W1);
pub const R0Y2W2: Genome3 = gen3(R0, Y2, W2);

pub const R1Y0W0: Genome3 = gen3(R1, Y0, W0);
pub const R1Y0W1: Genome3 = gen3(R1, Y0, W1);
pub const R1Y0W2: Genome3 = gen3(R1, Y0, W2);

pub const R1Y1W0: Genome3 = gen3(R1, Y1, W0);
pub const R1Y1W1: Genome3 = gen3(R1, Y1, W1);
pub const R1Y1W2: Genome3 = gen3(R1, Y1, W2);

pub const R1Y2W0: Genome3 = gen3(R1, Y2, W0);
pub const R1Y2W1: Genome3 = gen3(R1, Y2, W1);
pub const R1Y2W2: Genome3 = gen3(R1, Y2, W2);

pub const R2Y0W0: Genome3 = gen3(R2, Y0, W0);
pub const R2Y0W1: Genome3 = gen3(R2, Y0, W1);
pub const R2Y0W2: Genome3 = gen3(R2, Y0, W2);

pub const R2Y1W0: Genome3 = gen3(R2, Y1, W0);
pub const R2Y1W1: Genome3 = gen3(R2, Y1, W1);
pub const R2Y1W2: Genome3 = gen3(R2, Y1, W2);

pub const R2Y2W0: Genome3 = gen3(R2, Y2, W0);
pub const R2Y2W1: Genome3 = gen3(R2, Y2, W1);
pub const R2Y2W2: Genome3 = gen3(R2, Y2, W2);

// Genome 4 patterns
// B0
pub const R0Y0W0B0: Genome4 = gen4(R0, Y0, W0, B0);
pub const R0Y0W1B0: Genome4 = gen4(R0, Y0, W1, B0);
pub const R0Y0W2B0: Genome4 = gen4(R0, Y0, W2, B0);

pub const R0Y1W0B0: Genome4 = gen4(R0, Y1, W0, B0);
pub const R0Y1W1B0: Genome4 = gen4(R0, Y1, W1, B0);
pub const R0Y1W2B0: Genome4 = gen4(R0, Y1, W2, B0);

pub const R0Y2W0B0: Genome4 = gen4(R0, Y2, W0, B0);
pub const R0Y2W1B0: Genome4 = gen4(R0, Y2, W1, B0);
pub const R0Y2W2B0: Genome4 = gen4(R0, Y2, W2, B0);

pub const R1Y0W0B0: Genome4 = gen4(R1, Y0, W0, B0);
pub const R1Y0W1B0: Genome4 = gen4(R1, Y0, W1, B0);
pub const R1Y0W2B0: Genome4 = gen4(R1, Y0, W2, B0);

pub const R1Y1W0B0: Genome4 = gen4(R1, Y1, W0, B0);
pub const R1Y1W1B0: Genome4 = gen4(R1, Y1, W1, B0);
pub const R1Y1W2B0: Genome4 = gen4(R1, Y1, W2, B0);

pub const R1Y2W0B0: Genome4 = gen4(R1, Y2, W0, B0);
pub const R1Y2W1B0: Genome4 = gen4(R1, Y2, W1, B0);
pub const R1Y2W2B0: Genome4 = gen4(R1, Y2, W2, B0);

pub const R2Y0W0B0: Genome4 = gen4(R2, Y0, W0, B0);
pub const R2Y0W1B0: Genome4 = gen4(R2, Y0, W1, B0);
pub const R2Y0W2B0: Genome4 = gen4(R2, Y0, W2, B0);

pub const R2Y1W0B0: Genome4 = gen4(R2, Y1, W0, B0);
pub const R2Y1W1B0: Genome4 = gen4(R2, Y1, W1, B0);
pub const R2Y1W2B0: Genome4 = gen4(R2, Y1, W2, B0);

pub const R2Y2W0B0: Genome4 = gen4(R2, Y2, W0, B0);
pub const R2Y2W1B0: Genome4 = gen4(R2, Y2, W1, B0);
pub const R2Y2W2B0: Genome4 = gen4(R2, Y2, W2, B0);

// B1
pub const R0Y0W0B1: Genome4 = gen4(R0, Y0, W0, B1);
pub const R0Y0W1B1: Genome4 = gen4(R0, Y0, W1, B1);
pub const R0Y0W2B1: Genome4 = gen4(R0, Y0, W2, B1);

pub const R0Y1W0B1: Genome4 = gen4(R0, Y1, W0, B1);
pub const R0Y1W1B1: Genome4 = gen4(R0, Y1, W1, B1);
pub const R0Y1W2B1: Genome4 = gen4(R0, Y1, W2, B1);

pub const R0Y2W0B1: Genome4 = gen4(R0, Y2, W0, B1);
pub const R0Y2W1B1: Genome4 = gen4(R0, Y2, W1, B1);
pub const R0Y2W2B1: Genome4 = gen4(R0, Y2, W2, B1);

pub const R1Y0W0B1: Genome4 = gen4(R1, Y0, W0, B1);
pub const R1Y0W1B1: Genome4 = gen4(R1, Y0, W1, B1);
pub const R1Y0W2B1: Genome4 = gen4(R1, Y0, W2, B1);

pub const R1Y1W0B1: Genome4 = gen4(R1, Y1, W0, B1);
pub const R1Y1W1B1: Genome4 = gen4(R1, Y1, W1, B1);
pub const R1Y1W2B1: Genome4 = gen4(R1, Y1, W2, B1);

pub const R1Y2W0B1: Genome4 = gen4(R1, Y2, W0, B1);
pub const R1Y2W1B1: Genome4 = gen4(R1, Y2, W1, B1);
pub const R1Y2W2B1: Genome4 = gen4(R1, Y2, W2, B1);

pub const R2Y0W0B1: Genome4 = gen4(R2, Y0, W0, B1);
pub const R2Y0W1B1: Genome4 = gen4(R2, Y0, W1, B1);
pub const R2Y0W2B1: Genome4 = gen4(R2, Y0, W2, B1);

pub const R2Y1W0B1: Genome4 = gen4(R2, Y1, W0, B1);
pub const R2Y1W1B1: Genome4 = gen4(R2, Y1, W1, B1);
pub const R2Y1W2B1: Genome4 = gen4(R2, Y1, W2, B1);

pub const R2Y2W0B1: Genome4 = gen4(R2, Y2, W0, B1);
pub const R2Y2W1B1: Genome4 = gen4(R2, Y2, W1, B1);
pub const R2Y2W2B1: Genome4 = gen4(R2, Y2, W2, B1);

// B2
pub const R0Y0W0B2: Genome4 = gen4(R0, Y0, W0, B2);
pub const R0Y0W1B2: Genome4 = gen4(R0, Y0, W1, B2);
pub const R0Y0W2B2: Genome4 = gen4(R0, Y0, W2, B2);

pub const R0Y1W0B2: Genome4 = gen4(R0, Y1, W0, B2);
pub const R0Y1W1B2: Genome4 = gen4(R0, Y1, W1, B2);
pub const R0Y1W2B2: Genome4 = gen4(R0, Y1, W2, B2);

pub const R0Y2W0B2: Genome4 = gen4(R0, Y2, W0, B2);
pub const R0Y2W1B2: Genome4 = gen4(R0, Y2, W1, B2);
pub const R0Y2W2B2: Genome4 = gen4(R0, Y2, W2, B2);

pub const R1Y0W0B2: Genome4 = gen4(R1, Y0, W0, B2);
pub const R1Y0W1B2: Genome4 = gen4(R1, Y0, W1, B2);
pub const R1Y0W2B2: Genome4 = gen4(R1, Y0, W2, B2);

pub const R1Y1W0B2: Genome4 = gen4(R1, Y1, W0, B2);
pub const R1Y1W1B2: Genome4 = gen4(R1, Y1, W1, B2);
pub const R1Y1W2B2: Genome4 = gen4(R1, Y1, W2, B2);

pub const R1Y2W0B2: Genome4 = gen4(R1, Y2, W0, B2);
pub const R1Y2W1B2: Genome4 = gen4(R1, Y2, W1, B2);
pub const R1Y2W2B2: Genome4 = gen4(R1, Y2, W2, B2);

pub const R2Y0W0B2: Genome4 = gen4(R2, Y0, W0, B2);
pub const R2Y0W1B2: Genome4 = gen4(R2, Y0, W1, B2);
pub const R2Y0W2B2: Genome4 = gen4(R2, Y0, W2, B2);

pub const R2Y1W0B2: Genome4 = gen4(R2, Y1, W0, B2);
pub const R2Y1W1B2: Genome4 = gen4(R2, Y1, W1, B2);
pub const R2Y1W2B2: Genome4 = gen4(R2, Y1, W2, B2);

pub const R2Y2W0B2: Genome4 = gen4(R2, Y2, W0, B2);
pub const R2Y2W1B2: Genome4 = gen4(R2, Y2, W1, B2);
pub const R2Y2W2B2: Genome4 = gen4(R2, Y2, W2, B2);
