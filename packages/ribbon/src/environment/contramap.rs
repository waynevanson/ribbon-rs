use std::marker::PhantomData;

use super::Environment;

pub struct ContramapEnvironment<T, B, F>
where
    T: Environment,
    F: FnOnce(B) -> T::Environment,
{
    value: T,
    function: F,
    marker: PhantomData<B>,
}

impl<T, B, F> Environment for ContramapEnvironment<T, B, F>
where
    T: Environment,
    F: FnOnce(B) -> T::Environment,
{
    type Environment = B;
}

impl<T, B, F> ContramapEnvironment<T, B, F>
where
    T: Environment,
    F: FnOnce(B) -> T::Environment,
{
    pub fn new(value: T, function: F) -> ContramapEnvironment<T, B, F> {
        ContramapEnvironment {
            value,
            function,
            marker: PhantomData,
        }
    }
}
