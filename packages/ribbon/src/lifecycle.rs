/// https://stenciljs.com/docs/component-lifecycle

// renderer
pub trait Connect {
    type Value;

    // callback
    fn connected(&mut self, value: Self::Value) {}

    // callback
    fn disconnected(&mut self, value: Self::Value) {}
}

pub trait Load {
    // callback
    fn will_load(&mut self) {}

    // callback
    fn did_load(&mut self) {}
}

pub trait Render {
    // callback
    fn will_render(&mut self) {}

    // user to implement?
    // renderer to implement?
    // some kind of side effect that can error?
    fn render(&self) {}

    // callback
    fn did_render(&mut self) {}
}

pub trait Update {
    fn should_update(&self, previous: &Self) -> bool {
        false
    }

    // callback
    fn will_update(&mut self) {}

    // callback
    fn did_update(&mut self) {}
}
