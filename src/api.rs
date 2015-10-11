use rand::Rng;

/// Type that can be used to create a random instance of `Output`.
///
/// Since no state is recorded, each sample is (statistically)
/// independent of all others, assuming the `Rng` used has this
/// property.
pub trait Distribution {
    /// The type that this trait is produced by sampling from this distribution.
    type Output;
    /// Generate a random value of `Output`, using `rng` as the
    /// source of randomness.
    fn sample<R: Rng>(&self, rng: &mut R) -> <Self as Distribution>::Output;
}

/// Data types that have a default distribution for generating random values.
///
/// For example for integers, the default distribution is a uniform distribution over all possible
/// values.
pub trait DefaultDistribution {
    type Distribution: Distribution<Output=Self>;

    fn default_distribution() -> <Self as DefaultDistribution>::Distribution;
}

/// Data types that can be converted into a `Distribution` over type `T`.
///
/// This trait is a little different from a normal `Into` trait because the type parameter
/// isn't the type of the `Distribution` being converted into, but the type that is generated by
/// that distribution.  This is so that `gen(&mut rng, ..) -> T` will work without additional type
/// annotations for the distribution.
pub trait IntoDistribution<T>: Sized {
    type Distribution: Distribution<Output=T>;

    fn into_distribution(self) -> <Self as IntoDistribution<T>>::Distribution;

    fn sample<R: Rng>(self, rng: &mut R) -> T {
        self.into_distribution().sample(rng)
    }
}
