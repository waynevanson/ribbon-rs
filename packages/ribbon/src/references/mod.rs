pub trait References {
    type References;
    type Action;

    fn mutate(references: &mut Self::References, action: Self::Action);
}
