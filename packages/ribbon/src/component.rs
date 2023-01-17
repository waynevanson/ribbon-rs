use crate::vnode::VNode;

pub trait Stateless<T>
where
    T: Into<VNode>,
{
    fn view(&self) -> T;
}

pub trait Stateful<T>
where
    T: Into<VNode>,
    Self: Sized,
{
    type Message;

    fn view(&self) -> T;

    fn update(self, message: Self::Message) -> Option<Self>;
}
