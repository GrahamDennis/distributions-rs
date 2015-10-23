use core::{Distribution, IntoDistribution};
use uniform::Uniform;

use std::ops::{Range};
use std::num::Wrapping as w;
use rand::Rng;
use num::{Bounded};

pub struct UniformPrimitiveIntegerRange<T, TUnsigned> {
    low: T,
    range: TUnsigned,
    accept_zone: TUnsigned,
    base_distribution: Uniform<TUnsigned>
}

macro_rules! uniform_integer_range_impls {
    ($(($ty:ty, $tyUnsigned:ty)),*) => {
        $(
            impl UniformPrimitiveIntegerRange<$ty, $tyUnsigned>
            {
                #[inline]
                pub fn new(low: $ty, high: $ty) -> UniformPrimitiveIntegerRange<$ty, $tyUnsigned> {
                    assert!(low < high);
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
            }

            impl IntoDistribution<$ty> for Range<$ty>
            {
                type Distribution = UniformPrimitiveIntegerRange<$ty, $tyUnsigned>;

                #[inline]
                fn into_distribution(self) -> UniformPrimitiveIntegerRange<$ty, $tyUnsigned> {
                    UniformPrimitiveIntegerRange::<$ty, $tyUnsigned>::new(self.start, self.end)
                }
            }

            impl Distribution for UniformPrimitiveIntegerRange<$ty, $tyUnsigned>
            {
                type Output = $ty;

                #[inline]
                fn sample<R: Rng>(&self, rng: &mut R) -> $ty {
                    loop {
                        let v: $tyUnsigned = self.base_distribution.sample(rng);

                        if v < self.accept_zone {
                            return (w(self.low) + w((v % self.range) as $ty)).0;
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
        UniformIntegerRange::new(10, 10);
    }

    #[should_panic]
    #[test]
    fn test_range_bad_limits_flipped() {
        UniformIntegerRange::new(10, 5);
    }

    #[test]
    fn test_range_into_distribution() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

        let d: UniformIntegerRange<u8> = (1..10).into_distribution();
        let _: u8 = d.sample(&mut rng);
    }
}
