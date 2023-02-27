pub trait View {
    type Node;

    fn view<F>(&self) -> Self::Node;
}
