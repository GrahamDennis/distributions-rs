use core::{Distribution, IntoDistribution};

use rand::Rng;
use std;

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
#[derive(Copy, Clone)]
pub struct Constant<T>(pub T);

impl<T: Clone> Distribution for Constant<T> {
    type Output=T;

    #[inline]
    fn sample<R: Rng>(&self, _: &mut R) -> T {
        self.0.clone()
    }
}

macro_rules! into_constant_distribution {
    ($($ty:ty),* ) => {
        $(
            impl IntoDistribution<$ty> for $ty {
                type Distribution = Constant<Self>;

                #[inline]
                fn into_distribution(self) -> Constant<Self> {
                    Constant(self)
                }
            }
        )*
    }
}

into_constant_distribution! {
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
    bool, char
}

macro_rules! generic_into_constant_distribution {
    ($ty:ty, $($ident:ident),* ) => {
        impl <$($ident),*> IntoDistribution<$ty> for $ty where
            $ty: Clone {
            type Distribution = Constant<Self>;

            #[inline]
            fn into_distribution(self) -> Constant<Self> {
                Constant(self)
            }
        }
    }
}

generic_into_constant_distribution! { Vec<T>, T }
generic_into_constant_distribution! { std::collections::VecDeque<T>, T }
generic_into_constant_distribution! { std::collections::LinkedList<T>, T }
generic_into_constant_distribution! { std::collections::HashMap<K, V>, K, V }
generic_into_constant_distribution! { std::collections::BTreeMap<K, V>, K, V }
generic_into_constant_distribution! { std::collections::HashSet<T>, T }
generic_into_constant_distribution! { std::collections::BTreeSet<T>, T }
generic_into_constant_distribution! { std::collections::BinaryHeap<T>, T }


#[cfg(test)]
mod tests {
    use super::*;

    use core::{Distribution, IntoDistribution};
    use rand::{self, Rng, thread_rng};

    #[test]
    fn test_generate_u8() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

        let v: u8 =  Constant(42).sample(&mut rng);
        assert_eq!(v, 42);
    }

    #[test]
    fn test_into_distribution() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

        let v: u8 =  42u8.into_distribution().sample(&mut rng);
        assert_eq!(v, 42);
    }

    #[test]
    fn test_vec_into_distribution() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

        let v: Vec<u8> =  vec![42u8].into_distribution().sample(&mut rng);
        assert_eq!(v, vec![42u8]);
    }
}
