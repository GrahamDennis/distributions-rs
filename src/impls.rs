use core::{Distribution, IntoDistribution};
use uniform_range::UniformPrimitiveIntegerRange;

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

pub enum WeightedBool {
    AlwaysTrue,
    OneIn(UniformPrimitiveIntegerRange<u32>)
}

impl WeightedBool {
    pub fn new(n: u32) -> WeightedBool {
        if n <= 1 {
            WeightedBool::AlwaysTrue
        } else {
            WeightedBool::OneIn(UniformPrimitiveIntegerRange::new(0, n))
        }
    }
}

impl Distribution for WeightedBool {
    type Output = bool;

    #[inline]
    fn sample<R: Rng>(&self, r: &mut R) -> bool {
        match *self {
            WeightedBool::AlwaysTrue => true,
            WeightedBool::OneIn(d) => d.sample(r) == 0
        }
    }
}

pub struct RandomElement<'a, T: 'a> {
    values: &'a [T],
    range: UniformPrimitiveIntegerRange<usize>
}

impl <'a, T> RandomElement<'a, T> {
    pub fn from(values: &'a [T]) -> Option<RandomElement<'a, T>> {
        if values.len() == 0 {
            None
        } else {
            let range_distribution = UniformPrimitiveIntegerRange::new(0, values.len());
            Some(RandomElement { values: values, range: range_distribution })
        }
    }
}

impl <'a, T> Distribution for RandomElement<'a, T> {
    type Output = &'a T;

    #[inline]
    fn sample<R: Rng>(&self, r: &mut R) -> &'a T {
        &self.values[self.range.sample(r)]
    }
}

impl <'a, T: 'a> IntoDistribution<&'a T> for &'a [T] {
    type Distribution = RandomElement<'a, T>;

    #[inline]
    fn into_distribution(self) -> RandomElement<'a, T> {
        RandomElement::from(self).expect("Tried to convert an empty slice into a `Distribution`")
    }
}

pub struct Alphanum(RandomElement<'static, u8>);

impl Alphanum {
    pub fn new() -> Alphanum {
        const GEN_ASCII_STR_CHARSET: &'static [u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
              abcdefghijklmnopqrstuvwxyz\
              0123456789";
        Alphanum(RandomElement::from(GEN_ASCII_STR_CHARSET).unwrap())
    }
}

impl Distribution for Alphanum {
    type Output = char;

    #[inline]
    fn sample<R: Rng>(&self, r: &mut R) -> char {
        *self.0.sample(r) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::{Distribution, IntoDistribution};
    use rand::{self, Rng, thread_rng};
    use rng_ext::RngExt;

    fn create_rng() -> rand::XorShiftRng {
        rand::thread_rng().gen()
    }

    #[test]
    fn test_generate_u8() {
        let mut rng = create_rng();

        let v: u8 =  Constant(42).sample(&mut rng);
        assert_eq!(v, 42);
    }

    #[test]
    fn test_into_distribution() {
        let mut rng = create_rng();

        let v: u8 =  42u8.into_distribution().sample(&mut rng);
        assert_eq!(v, 42);
    }

    #[test]
    fn test_vec_into_distribution() {
        let mut rng = create_rng();

        let v: Vec<u8> =  vec![42u8].into_distribution().sample(&mut rng);
        assert_eq!(v, vec![42u8]);
    }

    #[test]
    fn test_weighted_bool_trivial() {
        let mut rng = create_rng();

        let d = WeightedBool::new(1);
        let v = d.sample(&mut rng);
        assert_eq!(v, true);
    }

    #[test]
    fn test_random_element_on_empty_vec() {
        let mut rng = create_rng();

        let v: Vec<usize> = vec![];
        let d = RandomElement::from(&v);
        let s = d.sample(&mut rng);

        assert_eq!(s, None);
    }

    #[test]
    fn test_alphanum() {
        let mut rng = create_rng();

        let d = Alphanum::new();
        let _: String = rng.generate_iter(&d).take(100).collect();
    }
}
