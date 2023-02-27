mod contramap;
use self::contramap::ContramapProps;

pub trait Props: Sized {
    type Props;

    fn contramap_props<F, B>(self, function: F) -> ContramapProps<Self, B, F>
    where
        F: FnOnce(B) -> Self::Props,
    {
        ContramapProps::new(self, function)
    }
}
