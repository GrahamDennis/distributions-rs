use core::{DefaultDistribution, Distribution};
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

#[derive(Copy, Clone)]
pub struct RandomSimpleDistribution<T>(marker::PhantomData<fn() -> T>);

impl <T: RandomSimple> RandomSimpleDistribution<T> {
    #[inline]
    pub fn new() -> RandomSimpleDistribution<T> {
        RandomSimpleDistribution(marker::PhantomData)
    }
}

impl <T: RandomSimple> Distribution for RandomSimpleDistribution<T> {
    type Output = T;

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> T {
        <T as RandomSimple>::random(rng)
    }
}

macro_rules! random_simple_to_default_distribution {
    ($ty:ty) => {
        impl DefaultDistribution for $ty {
            type Distribution = RandomSimpleDistribution<$ty>;

            #[inline]
            fn default_distribution() -> RandomSimpleDistribution<$ty> {
                RandomSimpleDistribution::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, thread_rng, Rng};
    use core::{Distribution, DefaultDistribution};

    struct MyType(u8);

    impl RandomSimple for MyType {
        fn random<R: rand::Rng>(_: &mut R) -> Self {
            MyType(0)
        }
    }

    random_simple_to_default_distribution! { MyType }

    fn create_rng() -> rand::XorShiftRng {
        rand::thread_rng().gen()
    }

    #[test]
    fn test_generate_u8() {
        let mut rng = create_rng();

        let d = MyType::default_distribution();
        let _: MyType =  d.sample(&mut rng);
    }
}
