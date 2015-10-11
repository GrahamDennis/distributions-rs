
use api::{IntoDistribution, DefaultDistribution};

use num::{Bounded, Integer};
use std::ops::{RangeFull, Range, RangeFrom, RangeTo, Add, Sub};

pub struct Uniform<T>(marker::PhantomData<fn() -> T>);

impl <T: Integer + Bounded + DefaultDistribution> IntoDistribution<X> for RangeFull {
    type Distribution = Uniform<T>;

    fn into_distribution(self) -> Uniform<T> {
        X::default_distribution()
    }
}

#[cfg(test)]
use rand::{self, thread_rng, Rng};

#[test]
fn test_generate_u8() {
    let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

    let _: u8 = RangeFull.sample(&mut rng);
}
