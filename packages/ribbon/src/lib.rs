pub mod component;
pub mod event;
pub mod index_dimensional;
pub mod node;

pub mod prelude {
    pub use crate::component::{Stateful, Stateless};
    pub use crate::node::VNode;
}
