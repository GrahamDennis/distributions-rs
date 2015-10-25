use core::{Distribution, IntoDistribution};
use uniform::Uniform;

use std::ops::{Range};
use std::num::Wrapping as w;
use rand::Rng;
use num::{Bounded, Integer};

pub trait CreateRange: Integer {
    type Unsigned;
}

pub trait UniformPrimitiveIntegerRangeTrait<T: CreateRange> {
    fn new(low: T, high: T) -> UniformPrimitiveIntegerRange<T>;
    fn sample<R: Rng>(d: &UniformPrimitiveIntegerRange<T>, rng: &mut R) -> T;
}

pub struct UniformPrimitiveIntegerRange<T: CreateRange> {
    low: T,
    range: T::Unsigned,
    accept_zone: T::Unsigned,
    base_distribution: Uniform<T::Unsigned>
}

impl <T: CreateRange> UniformPrimitiveIntegerRange<T> where
    UniformPrimitiveIntegerRange<T>: UniformPrimitiveIntegerRangeTrait<T>
{
    #[inline]
    fn new(low: T, high: T) -> UniformPrimitiveIntegerRange<T> {
        assert!(low < high);
        <Self as UniformPrimitiveIntegerRangeTrait<T>>::new(low, high)
    }
}

impl <T> IntoDistribution<T> for Range<T> where
    T: CreateRange,
    UniformPrimitiveIntegerRange<T>: Distribution<Output=T> + UniformPrimitiveIntegerRangeTrait<T>
{
    type Distribution = UniformPrimitiveIntegerRange<T>;

    #[inline]
    fn into_distribution(self) -> UniformPrimitiveIntegerRange<T> {
        UniformPrimitiveIntegerRange::new(self.start, self.end)
    }
}

impl <T> Distribution for UniformPrimitiveIntegerRange<T> where
    T: CreateRange,
    UniformPrimitiveIntegerRange<T>: UniformPrimitiveIntegerRangeTrait<T>
{
    type Output = T;

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> T {
        <Self as UniformPrimitiveIntegerRangeTrait<T>>::sample(self, rng)
    }
}

macro_rules! uniform_integer_range_impls {
    ($(($ty:ty, $tyUnsigned:ty)),*) => {
        $(
            impl CreateRange for $ty {
                type Unsigned = $tyUnsigned;
            }

            impl UniformPrimitiveIntegerRangeTrait<$ty> for UniformPrimitiveIntegerRange<$ty> {
                #[inline]
                fn new(low: $ty, high: $ty) -> UniformPrimitiveIntegerRange<$ty> {
                    let unsigned_range = (w(high as $tyUnsigned) - w(low as $tyUnsigned)).0;
                    let unsigned_max: $tyUnsigned = Bounded::max_value();
                    let unsigned_zone = unsigned_max - (unsigned_max % unsigned_range);

                    UniformPrimitiveIntegerRange {
                        low: low,
                        range: unsigned_range,
                        accept_zone: unsigned_zone,
                        base_distribution: Uniform::new()
                    }
                }

                #[inline]
                fn sample<R: Rng>(d: &UniformPrimitiveIntegerRange<$ty>, rng: &mut R) -> $ty {
                    loop {
                        let v: $tyUnsigned = d.base_distribution.sample(rng);

                        if v < d.accept_zone {
                            return (w(d.low) + w((v % d.range) as $ty)).0;
                        }
                    }
                }

            }
        )*
    }
}

macro_rules! signed_and_unsigned_range_impls {
    ($(($ty:ty, $tyUnsigned:ty)),*) => {
        uniform_integer_range_impls! { $(($ty, $tyUnsigned), ($tyUnsigned, $tyUnsigned)),* }
    }
}

signed_and_unsigned_range_impls! {
    (i8, u8), (i16, u16), (i32, u32), (i64, u64), (isize, usize)
}


#[cfg(test)]
mod tests {
    use super::*;
    use core::{Distribution, IntoDistribution};

    use rand::{self, thread_rng, Rng};

    #[should_panic]
    #[test]
    fn test_range_bad_limits_equal() {
        UniformPrimitiveIntegerRange::new(10, 10);
    }

    #[should_panic]
    #[test]
    fn test_range_bad_limits_flipped() {
        UniformPrimitiveIntegerRange::new(10, 5);
    }

    #[test]
    fn test_range_into_distribution() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();
        let d = (1..10).into_distribution();
        let _: u8 = d.sample(&mut rng);
    }
}
