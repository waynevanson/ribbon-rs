pub mod component;
pub mod event;
pub mod index_dimensional;
pub mod vdom;
pub mod vnode;

pub mod prelude {
    pub use crate::component::{Stateful, Stateless};
    pub use crate::vnode::VNode;
}
