extern crate rand;
extern crate num;

mod api;
mod random;
mod impls;
pub use api::{Distribution, DefaultDistribution, IntoDistribution};
pub use random::{Random, RandomSimple, RandomSimpleDistribution};
pub use impls::{Constant};
pub mod uniform;
pub mod uniform_range;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_default_distribution() {
        u8::default_distribution();
    }
}
