use rand::Rng;

use core::{Distribution, IntoDistribution};

pub trait RngExt: Rng {
    #[inline]
    fn generate<T, IntoD: IntoDistribution<T>>(&mut self, d: IntoD) ->
        T where
        Self: Sized
    {
        d.into_distribution().sample(self)
    }

    #[inline]
    fn generate_iter<'a, T, IntoD: IntoDistribution<T>>(&'a mut self, d: IntoD) ->
        GenIter<'a, Self, <IntoD as IntoDistribution<T>>::Distribution> where
        Self: Sized
    {
        GenIter {
            rng: self,
            distribution: d.into_distribution()
        }
    }
}

pub struct GenIter<'a, R, D> where R: 'a, D: 'a {
    rng: &'a mut R,
    distribution: D
}

impl <'a, R, D> Iterator for GenIter<'a, R, D> where R: 'a + Rng, D: 'a + Distribution {
    type Item = <D as Distribution>::Output;

    #[inline]
    fn next(&mut self) -> Option<<D as Distribution>::Output> {
        Some(self.distribution.sample(self.rng))
    }
}

impl <R: Rng> RngExt for R {}

#[cfg(test)]
mod tests {
    use super::*;
    use uniform::Uniform;

    use rand::{self, thread_rng, Rng};

    fn create_rng() -> rand::XorShiftRng {
        rand::thread_rng().gen()
    }

    #[test]
    fn test_generate_u8() {
        let mut rng = create_rng();

        let _ : u8 = rng.generate(1..10);
        let _ : u8 = rng.generate(..);
    }

    #[test]
    fn test_generate_from_distribution() {
        let mut rng = create_rng();

        let d = Uniform::<u8>::new();
        let _: u8 = rng.generate(&d);
        let _: u8 = rng.generate(&d);
    }
}
