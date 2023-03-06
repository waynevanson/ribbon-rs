pub mod dispatcher;
use crate::application::vnode::VNode;

use self::dispatcher::Dispatcher;

use super::environment::Environment;

// instead to ToNode, there should be a Node trait
// that provides access to all the data a Node in a VDOM should have.
pub trait ToNode: Sized {
    type Environment;

    fn to_node(self) -> VNode;
}

pub trait View: Environment + Sized {
    type Node: ToNode<Environment = Self::Environment>;

    fn view(&self, dispatcher: Dispatcher<Self>) -> Self::Node;
}
