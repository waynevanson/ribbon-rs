use super::Props;

pub struct ProvidedProps<T>
where
    T: Props,
{
    value: T,
    props: T::Props,
}

impl<T> ProvidedProps<T>
where
    T: Props,
{
    pub fn new(value: T, props: T::Props) -> Self {
        Self { value, props }
    }
}
