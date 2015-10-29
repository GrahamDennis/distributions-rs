use core::{Distribution, IntoDistribution};
use uniform::Uniform;

use std::ops::{Range, Add, Sub};
use std::num::Wrapping;
use rand::Rng;
use num::{Bounded, PrimInt};

trait PrimitiveInteger: PrimInt {
    type Unsigned : PrimitiveInteger;

    fn to_unsigned(self) -> <Self as PrimitiveInteger>::Unsigned;
    fn from_unsigned(u: <Self as PrimitiveInteger>::Unsigned) -> Self;
}

#[derive(Copy, Clone)]
pub struct UniformPrimitiveIntegerRange<T> {
    low: T,
    range: T,
    accept_zone: T,
    base_distribution: Uniform<T>
}

impl <T: PrimitiveInteger> UniformPrimitiveIntegerRange<T>
    where   Wrapping<T::Unsigned>: Sub<Output=Wrapping<T::Unsigned>>
{
    #[inline]
    pub fn new(low: T, high: T) -> Option<UniformPrimitiveIntegerRange<T>> {
        if !(low < high) {
            None
        } else {
            let unsigned_range = (Wrapping(high.to_unsigned()) - Wrapping(low.to_unsigned())).0;
            let unsigned_max: T::Unsigned = Bounded::max_value();
            let unsigned_zone = unsigned_max - (unsigned_max % unsigned_range);

            Some(UniformPrimitiveIntegerRange {
                low: low,
                range: T::from_unsigned(unsigned_range),
                accept_zone: T::from_unsigned(unsigned_zone),
                base_distribution: Uniform::new()
            })
        }
    }
}

impl <T: PrimitiveInteger> IntoDistribution<T> for Range<T>
    where   Wrapping<T::Unsigned>: Sub<Output=Wrapping<T::Unsigned>>,
            UniformPrimitiveIntegerRange<T>: Distribution<Output=T>
{
    type Distribution = UniformPrimitiveIntegerRange<T>;

    #[inline]
    fn into_distribution(self) -> UniformPrimitiveIntegerRange<T> {
        UniformPrimitiveIntegerRange::new(self.start, self.end).unwrap()
    }
}

impl <T: PrimitiveInteger> Distribution for UniformPrimitiveIntegerRange<T>
    where   Wrapping<T::Unsigned>: Add<Output=Wrapping<T::Unsigned>>,
            Uniform<T>: Distribution<Output=T>
{
    type Output = T;

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> T {
        loop {
            let v = self.base_distribution.sample(rng).to_unsigned();

            if v < (self.accept_zone.to_unsigned()) {
                return T::from_unsigned((
                    Wrapping(self.low.to_unsigned()) +
                    Wrapping(v % self.range.to_unsigned())
                ).0);
            }
        }
    }
}

macro_rules! uniform_integer_range_impls {
    ($(($ty:ty, $tyUnsigned:ty)),*) => {
        $(
            impl PrimitiveInteger for $ty {
                type Unsigned = $tyUnsigned;

                #[inline]
                fn to_unsigned(self) -> $tyUnsigned {
                    self as $tyUnsigned
                }

                #[inline]
                fn from_unsigned(u: $tyUnsigned) -> Self {
                    u as $ty
                }
            }
        )*
    }
}

macro_rules! signed_and_unsigned_range_impls {
    ($(($tySigned:ty, $tyUnsigned:ty)),*) => {
        uniform_integer_range_impls! {
            $(
                ($tySigned, $tyUnsigned),
                ($tyUnsigned, $tyUnsigned)
            ),*
        }
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

    #[test]
    fn test_range_bad_limits_equal() {
        let d = UniformPrimitiveIntegerRange::new(10, 10);
        assert!(d.is_none());
    }

    #[test]
    fn test_range_bad_limits_flipped() {
        let d = UniformPrimitiveIntegerRange::new(10, 5);
        assert!(d.is_none());
    }

    #[test]
    fn test_range_into_distribution() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();
        let d = (1..10).into_distribution();
        let _: u8 = d.sample(&mut rng);
    }
}
