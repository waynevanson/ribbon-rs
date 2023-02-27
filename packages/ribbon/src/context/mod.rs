mod contramap;

use self::contramap::ContramapContext;

pub trait Context: Sized {
    type Context;

    fn contramap_environment<B, F>(self, function: F) -> ContramapContext<Self, B, F>
    where
        F: FnOnce(B) -> Self::Context,
    {
        ContramapContext::new(self, function)
    }
}
