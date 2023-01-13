use std::collections::HashMap;

use crate::index_dimensional::IndexDimensional;

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
        index.iter().try_fold(self, |vnode, indice| match vnode {
            VNode::Text { value: _ } => None,
            VNode::Element { children, .. } => children.iter().nth(*indice),
        })
    }

    pub fn iter_with_index<'a>(&'a self) -> DepthIterWithIndex<'a> {
        DepthIterWithIndex {
            node: self,
            index: Default::default(),
        }
    }
}

pub struct DepthIterWithIndex<'a> {
    node: &'a VNode,
    index: IndexDimensional,
}

impl<'a> Iterator for DepthIterWithIndex<'a> {
    type Item = (IndexDimensional, &'a VNode);

    // todo - can we make cloning conditional?
    // inrementing mutable index up then down seems like a bad pattern.
    fn next(&mut self) -> Option<Self::Item> {
        self.index
            .clone()
            .increment_step()
            .and_then(|last| {
                self.node
                    .nth(&last)
                    .map(|vnode| {
                        self.index = last;
                        vnode
                    })
                    .or_else(|| self.node.nth(self.index.increment_depth_mut()))
            })
            .or_else(|| self.node.nth(self.index.increment_depth_mut()))
            .map(|vnode| (self.index.clone(), vnode))
    }
}
