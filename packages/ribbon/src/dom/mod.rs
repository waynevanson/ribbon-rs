// vnode, vdom
// graph
// diff

// hide each distinct environment behind feature flags so binaries are smaller.

pub trait DOM {
    fn create(&self, component: ()) -> Self;

    fn update(&mut self);

    fn listen(&mut self) {
        self.update()
    }
}

// state, memoization, effects

// dom should implement ToGraph, and that's where we control it.
// node should implement ToGraph.
pub trait ToGraph {
    type Node;
}
