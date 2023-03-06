mod contramap;
mod provided;

use self::{contramap::ContramapProps, provided::ProvidedProps};

pub trait Props: Sized {
    type Props;

    fn provide_props(self, props: Self::Props) -> ProvidedProps<Self> {
        ProvidedProps::new(self, props)
    }

    fn contramap_props<F, B>(self, function: F) -> ContramapProps<Self, B, F>
    where
        F: FnOnce(B) -> Self::Props,
    {
        ContramapProps::new(self, function)
    }
}
