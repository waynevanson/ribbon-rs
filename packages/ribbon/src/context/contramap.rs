use super::Context;
use std::marker::PhantomData;

pub struct ContramapContext<T, B, F>
where
    T: Context,
    F: FnOnce(B) -> T::Context,
{
    value: T,
    function: F,
    marker: PhantomData<B>,
}

impl<T, B, F> ContramapContext<T, B, F>
where
    T: Context,
    F: FnOnce(B) -> T::Context,
{
    pub fn new(value: T, function: F) -> Self {
        Self {
            value,
            function,
            marker: PhantomData,
        }
    }
}

impl<T, B, F> Context for ContramapContext<T, B, F>
where
    T: Context,
    F: FnOnce(B) -> T::Context,
{
    type Context = B;
}
