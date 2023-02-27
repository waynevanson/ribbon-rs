pub trait State {
    type State;
    type Action;

    fn update(state: Self::State, action: Self::Action) -> Self::State;
}
