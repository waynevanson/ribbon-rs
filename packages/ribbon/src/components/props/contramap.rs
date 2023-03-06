use super::Props;
use std::marker::PhantomData;

pub struct ContramapProps<T, B, F>
where
    T: Props,
    F: FnOnce(B) -> T::Props,
{
    value: T,
    function: F,
    marker: PhantomData<B>,
}

impl<T, B, F> ContramapProps<T, B, F>
where
    T: Props,
    F: FnOnce(B) -> T::Props,
{
    pub fn new(value: T, function: F) -> Self {
        Self {
            value,
            function,
            marker: PhantomData,
        }
    }
}

impl<T, B, F> Props for ContramapProps<T, B, F>
where
    T: Props,
    F: FnOnce(B) -> T::Props,
{
    type Props = B;
}
