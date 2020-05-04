use crate::flower::Flower;

struct Distribution<F> {
    probs: Vec<(F, usize)>,
}

impl<F: Flower> Distribution {
    pub fn breed(first: F, second: F) -> Self {
        unimplemented!();
    }
}
