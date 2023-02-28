pub trait References {
    type References;
    type Action;

    /// Called when a `view` calls `dispatcher.mutate()`
    fn mutate(references: &mut Self::References, action: Self::Action);
}
