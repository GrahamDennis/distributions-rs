extern crate rand;
extern crate num;

mod api;
mod random;
mod impls;
pub use api::{Distribution, DefaultDistribution, IntoDistribution};
pub use random::{Random, RandomSimple, RandomSimpleDistribution};
pub use impls::{Constant};
//pub mod uniform;

#[test]
fn it_works() {
    //u8::default_distribution();
}
