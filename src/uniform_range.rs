use api::{Distribution, IntoDistribution};
use uniform::Uniform;

use std::ops::{Range};
use rand::Rng;
use num::{Integer, Bounded};

pub struct UniformIntegerRange<T> {
    low: T,
    range: T,
    accept_zone: T,
    base_distribution: Uniform<T>
}

impl <X: Integer + Bounded + Copy> UniformIntegerRange<X> where
    Uniform<X>: Distribution<X>
{
    #[inline]
    pub fn new(low: X, high: X) -> UniformIntegerRange<X> {
        assert!(low < high);
        let range = high - low;
        let max: X = Bounded::max_value();
        let zone = max - (max % range);

        UniformIntegerRange {
            low: low,
            range: range,
            accept_zone: zone,
            base_distribution: Uniform::new()
        }
    }
}

impl <X: Integer + Bounded + Copy> IntoDistribution<X> for Range<X> where
    Uniform<X>: Distribution<X>
{
    type Distribution = UniformIntegerRange<X>;

    #[inline]
    fn into_distribution(self) -> UniformIntegerRange<X> {
        UniformIntegerRange::new(self.start, self.end)
    }
}

impl <X: Integer + Copy> Distribution<X> for UniformIntegerRange<X> where
    Uniform<X>: Distribution<X>
{
    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> X {
        loop {
            let v: X = self.base_distribution.sample(rng);

            if v < self.accept_zone {
                return self.low + (v % self.range);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::{Distribution, IntoDistribution};

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
