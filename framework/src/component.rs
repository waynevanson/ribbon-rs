use crate::node::VNode;

pub trait Stateless<T>
where
    T: Into<VNode>,
{
    fn view(&self) -> T;
}
