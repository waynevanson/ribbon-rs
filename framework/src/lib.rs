extern crate web_sys;

pub mod component;
pub mod html;
pub mod index_dimensional;
pub mod node;

pub mod prelude {
    pub use crate::component::Stateless;
    pub use crate::node::VNode;
}
