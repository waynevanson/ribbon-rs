use std::marker::PhantomData;

use crate::components::{references::References, state::State};

pub struct Dispatcher<T> {
    marker: PhantomData<T>,
}

impl<T> Dispatcher<T>
where
    T: State,
{
    /// Update the state using the actions provided.
    /// Calling this cause a rerender.
    ///
    /// To avoid triggering a rerender, please see `dispatcher.mutate`.
    pub fn update(&self, action: T::Action) {}
}

impl<T> Dispatcher<T>
where
    T: References,
{
    /// Update the state using the actions provided.
    /// Calling this will not cause a rerender.
    ///
    /// To trigger a rerender, please see `dispatcher.update`.
    pub fn mutate(&self, action: T::Action) {}
}
