use crate::index_dimensional::IndexDimensional;
use graph::prelude::*;
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

impl Graph<usize> for VNode {
    fn edge_count(&self) -> usize {
        self.children()
            .map(|vnodes| vnodes.iter().map(|vnode| vnode.edge_count()).sum())
            .unwrap_or(0)
    }

    fn node_count(&self) -> usize {
        1 + self
            .children()
            .map(|vnodes| vnodes.iter().map(|vnode| vnode.node_count()).sum())
            .unwrap_or(0)
    }
}

impl VNode {
    pub fn children(&self) -> Option<&[Self]> {
        match self {
            Self::Text { value: _ } => None,
            Self::Element { children, .. } => Some(children.as_slice()),
        }
    }

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

    pub fn child(&self, indice: usize) -> Option<&Self> {
        match self {
            VNode::Text { value: _ } => None,
            VNode::Element { children, .. } => children.iter().nth(indice),
        }
    }
}
