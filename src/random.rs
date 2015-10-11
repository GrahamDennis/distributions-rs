use api::{DefaultDistribution, Distribution};
use rand::Rng;
use std::marker;

/// Data types that can be created randomly.
///
/// This is a replacement for the current `Rand` trait.
pub trait Random {
    fn random<R: Rng>(rng: &mut R) -> Self;
}

impl <T: DefaultDistribution> Random for T
{
    #[inline]
    fn random<R: Rng>(rng: &mut R) -> Self {
        <Self as DefaultDistribution>::default_distribution().sample(rng)
    }
}


pub trait RandomSimple {
    fn random<R: Rng>(rng: &mut R) -> Self;
}

pub struct RandomSimpleDistribution<T>(marker::PhantomData<fn() -> T>);

impl <T: RandomSimple> Distribution for RandomSimpleDistribution<T> {
    type Output = T;

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> T {
        <T as RandomSimple>::random(rng)
    }
}

impl <T: RandomSimple> DefaultDistribution for T
{
    type Distribution = RandomSimpleDistribution<T>;

    #[inline]
    fn default_distribution() -> <Self as DefaultDistribution>::Distribution {
        RandomSimpleDistribution(marker::PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, thread_rng, Rng};
    use api::{Distribution, DefaultDistribution};

    struct MyType(u8);

    impl RandomSimple for MyType {
        fn random<R: rand::Rng>(_: &mut R) -> Self {
            MyType(0)
        }
    }

    #[test]
    fn test_generate_u8() {
        let mut rng: rand::XorShiftRng = rand::thread_rng().gen();

        let d = MyType::default_distribution();
        let _: MyType =  d.sample(&mut rng);
    }
}
