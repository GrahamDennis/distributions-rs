use core::{Distribution, DefaultDistribution, IntoDistribution};

use std::ops::{RangeFull};
use std::marker;
use std::mem;

use rand::Rng;
use num::PrimInt;

#[derive(Copy, Clone)]
pub struct Uniform<T> (marker::PhantomData<fn() -> T>);

trait DowncastPrimitiveInteger: PrimInt {
    fn from_u32(u: u32) -> Self;
    fn from_u64(u: u64) -> Self;
}

impl <T> Uniform<T> {
    #[inline]
    pub fn new() -> Uniform<T> {
        Uniform(marker::PhantomData)
    }
}

impl <T: DowncastPrimitiveInteger> IntoDistribution<T> for RangeFull
{
    type Distribution = Uniform<T>;

    #[inline]
    fn into_distribution(self) -> Uniform<T> {
        Uniform::new()
    }
}

impl <T: DowncastPrimitiveInteger> Distribution for Uniform<T>
{
    type Output = T;

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> T {
        if mem::size_of::<T>() <= 4 {
            T::from_u32(rng.next_u32())
        } else {
            T::from_u64(rng.next_u64())
        }
    }
}

macro_rules! integer_impls {
    ($($ty:ty),*) => {
        $(
            impl DowncastPrimitiveInteger for $ty {
                #[inline]
                fn from_u32(u: u32) -> $ty {
                    u as $ty
                }

                #[inline]
                fn from_u64(u: u64) -> $ty {
                    u as $ty
                }
            }
        )*
    }
}

integer_impls! {
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize
}

impl Distribution for Uniform<bool> {
    type Output = bool;

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> bool {
        Uniform::<u8>::new().sample(rng) & 1 == 1
    }
}

macro_rules! default_distribution_is_uniform {
    ($($ty:ty),* ) => {
        $(
            impl DefaultDistribution for $ty {
                type Distribution = Uniform<Self>;

                #[inline]
                fn default_distribution() -> Uniform<Self> {
                    Uniform::new()
                }
            }
        )*
    }
}

default_distribution_is_uniform! { isize, i8, i16, i32, i64,
                                   usize, u8, u16, u32, u64,
                                   bool }

#[cfg(test)]
mod tests {
    use super::*;

    use core::{Distribution, IntoDistribution};

    use rand::{self, thread_rng, Rng};
    use std::ops::{RangeFull};

    fn create_rng() -> rand::XorShiftRng {
        rand::thread_rng().gen()
    }

    #[test]
    fn test_generate_u8() {
        let mut rng = create_rng();

        fn foo<D: IntoDistribution<T>, T, R: Rng>(d: D, rng: &mut R) -> T {
            d.into_distribution().sample(rng)
        }

        let _: u8 = foo(RangeFull, &mut rng);
    }

    #[test]
    fn test_generate_u8_with_type_annotation() {
        let mut rng = create_rng();

        let d = IntoDistribution::<u8>::into_distribution(RangeFull);
        let _: u8 = d.sample(&mut rng);
    }

    #[test]
    fn test_range_full_into_distribution() {
        let mut rng = create_rng();

        let d: Uniform<u8> = (..).into_distribution();
        let _: u8 = d.sample(&mut rng);
    }
}
