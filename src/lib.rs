extern crate rand;
extern crate num;

mod core;
mod random;
mod impls;
mod rng_ext;
mod uniform;
mod uniform_range;

pub use core::{Distribution, DefaultDistribution, IntoDistribution};
pub use random::{Random, RandomSimple, RandomSimpleDistribution};
pub use impls::{Constant, WeightedBool, RandomElement, Alphanum};
pub use rng_ext::RngExt;
pub use uniform::Uniform;
pub use uniform_range::UniformPrimitiveIntegerRange;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_default_distribution() {
        u8::default_distribution();
    }
}
