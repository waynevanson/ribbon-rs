pub trait Renderer {
    type Environment;

    fn render();

    fn paint();
    fn hydrate();

    fn update();
}
