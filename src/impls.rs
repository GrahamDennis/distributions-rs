use core::{Distribution};

use rand::Rng;

/// A distribution that always returns the same value.
///
/// # Example
///
/// ```rust
/// # extern crate rand;
/// # extern crate distributions;
/// # fn main() {
///     use distributions::Distribution;
///
///     let d = distributions::Constant(42);
///     let v = d.sample(&mut rand::thread_rng());
///     assert_eq!(v, 42);
/// # }
/// ```
pub struct Constant<T>(pub T);

impl<T: Clone> Distribution for Constant<T> {
    type Output = T;

    #[inline]
    fn sample<R: Rng>(&self, _: &mut R) -> T {
        self.0.clone()
    }
}

#[cfg(test)]
use rand::{self, thread_rng};

#[test]
fn test_generate_u8() {
    let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

    let v: u8 =  Constant(42).sample(&mut rng);
    assert_eq!(v, 42);
}
