mod iter;

use self::iter::DepthIterWithIndex;
use crate::index_dimensional::IndexDimensional;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum VNode {
    Text {
        value: String,
    },
    Element {
        tag: String,
        attributes: HashMap<String, String>,
        children: Vec<VNode>,
    },
}

impl VNode {
    pub fn text(str: &str) -> Self {
        Self::Text {
            value: str.to_string(),
        }
    }

    pub fn element<C, T, A, S>(tag: &str, attributes: A, children: C) -> Self
    where
        C: IntoIterator<Item = T>,
        T: Into<Self>,
        A: IntoIterator<Item = S>,
        S: Into<(String, String)>,
    {
        Self::Element {
            tag: tag.to_string(),
            attributes: attributes.into_iter().map(|x| x.into()).collect(),
            children: children.into_iter().map(|x| x.into()).collect(),
        }
    }

    pub fn nth<'a>(&'a self, index: &IndexDimensional) -> Option<&'a Self> {
        index
            .iter()
            .try_fold(self, |vnode, indice| vnode.child(*indice))
    }

    pub fn iter_with_index<'a>(&'a self) -> DepthIterWithIndex<'a> {
        self.into_iter()
    }

    pub fn child(&self, indice: usize) -> Option<&Self> {
        match self {
            VNode::Text { value: _ } => None,
            VNode::Element { children, .. } => children.iter().nth(indice),
        }
    }
}

pub trait ToVNode {
    fn to_vnode(self) -> VNode;
}

impl<T> ToVNode for T
where
    T: Into<VNode>,
{
    fn to_vnode(self) -> VNode {
        self.into()
    }
}
