use core::{Distribution, DefaultDistribution, IntoDistribution};

use std::ops::{RangeFull};
use std::marker;
use rand::Rng;

pub struct Uniform<T> (marker::PhantomData<fn() -> T>);

impl <T> Uniform<T> where Uniform<T>: Distribution<Output=T> {
    #[inline]
    pub fn new() -> Uniform<T> {
        Uniform(marker::PhantomData)
    }
}

impl <T> IntoDistribution<T> for RangeFull where Uniform<T>: Distribution<Output=T> {
    type Distribution = Uniform<T>;

    #[inline]
    fn into_distribution(self) -> Uniform<T> {
        Uniform::new()
    }
}

macro_rules! integer_size_impls {
    ($mod_name:ident, $ty:ty, $ty32:ty, $ty64:ty) => {
        mod $mod_name {
            use rand::Rng;
            use core::Distribution;
            use std::mem;
            use super::Uniform;

            impl Distribution for Uniform<$ty> {
                type Output = $ty;

                #[inline]
                fn sample<R: Rng>(&self, rng: &mut R) -> $ty {
                    if mem::size_of::<$ty>() == 4 {
                        Uniform::<$ty32>::new().sample(rng) as $ty
                    } else {
                        Uniform::<$ty64>::new().sample(rng) as $ty
                    }
                }
            }
        }
    }
}

integer_size_impls! { isize_uniform_impls, isize, i32, i64 }
integer_size_impls! { usize_uniform_impls, usize, u32, u64 }

macro_rules! integer_impls {
    ($mod_name:ident, $ty:ty, $method_name:ident) => {
        mod $mod_name {
            use rand::Rng;
            use core::Distribution;
            use super::Uniform;

            impl Distribution for Uniform<$ty> {
                type Output = $ty;

                #[inline]
                fn sample<R: Rng>(&self, rng: &mut R) -> $ty {
                    rng.$method_name() as $ty
                }
            }
        }
    }
}

integer_impls! { i8_uniform_impls,  i8,  next_u32 }
integer_impls! { i16_uniform_impls, i16, next_u32 }
integer_impls! { i32_uniform_impls, i32, next_u32 }
integer_impls! { i64_uniform_impls, i64, next_u64 }
integer_impls! { u8_uniform_impls,  u8,  next_u32 }
integer_impls! { u16_uniform_impls, u16, next_u32 }
integer_impls! { u32_uniform_impls, u32, next_u32 }
integer_impls! { u64_uniform_impls, u64, next_u64 }

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

    #[test]
    fn test_generate_u8() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

        let _: u8 = RangeFull.sample(&mut rng);
    }

    #[test]
    fn test_range_full_into_distribution() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

        let d: Uniform<u8> = (..).into_distribution();
        let _: u8 = d.sample(&mut rng);
    }
}
