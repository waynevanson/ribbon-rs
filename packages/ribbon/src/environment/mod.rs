use self::contramap::ContramapEnvironment;

mod contramap;

pub trait Environment: Sized {
    type Environment;

    fn contramap_environment<B, F>(self, function: F) -> ContramapEnvironment<Self, B, F>
    where
        F: FnOnce(B) -> Self::Environment,
    {
        ContramapEnvironment::new(self, function)
    }
}
