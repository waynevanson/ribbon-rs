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

// when the value changes, should update and rerender.
pub struct UseState<A> {
    state: A,
}

impl<A> Update for UseState<A>
where
    A: PartialEq,
{
    fn should_update(&self, previous: &Self) -> bool {
        self.state == previous.state
    }
}

impl<A> UseState<A> {
    pub fn new(state: A) -> Self {
        UseState { state }
    }
}

// in our world, ref is state so there's no need to hold it elsewhere.
// however,
// can only be constructed from a component?
pub struct UseRef<'a, A> {
    reference: Option<&'a A>,
}

impl<'a, A> Connect for UseRef<'a, A> {
    type Value = &'a A;

    fn connected(&mut self, value: Self::Value) {
        self.reference = Some(value)
    }

    fn disconnected(&mut self, _value: Self::Value) {
        self.reference = None;
    }
}
