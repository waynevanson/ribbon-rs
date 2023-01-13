use std::{collections::HashMap, iter::once, ops::Deref};

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
        index
            .iter()
            .try_fold(self, |vnode, indice| vnode.child(*indice))
    }

    pub fn iter_with_index<'a>(&'a self) -> DepthIterWithIndex<'a> {
        DepthIterWithIndex {
            node: self,
            index: Default::default(),
        }
    }

    pub fn child(&self, indice: usize) -> Option<&Self> {
        match self {
            VNode::Text { value: _ } => None,
            VNode::Element { children, .. } => children.iter().nth(indice),
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
        let index = self.index.clone();
        let vnode = self.node.nth(&index)?;
        let current = (index, vnode);

        self.index = self
            .index
            .clone()
            .inner
            .split_last()
            .and_then(|(last, rest)| {
                let parent_of_last = rest
                    .iter()
                    .try_fold(self.node, |vnode, indice| vnode.child(*indice))?;

                let incremented = last + 1;
                let is_step = parent_of_last.child(incremented).is_some();
                let index = if is_step {
                    rest.into_iter()
                        .chain([&incremented])
                        .map(|x| x.to_owned())
                        .collect::<Vec<_>>()
                } else {
                    rest.into_iter()
                        .chain([last, &0])
                        .map(|x| x.to_owned())
                        .collect::<Vec<_>>()
                };

                Some(index.into())
            })
            .unwrap_or_else(|| vec![0].into());

        Some(current)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_iterate_values_in_order() {
        let element = VNode::element(
            "first",
            [("one".to_string(), "two".to_string())],
            vec![VNode::text("child")],
        );

        let indicies = element
            .iter_with_index()
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let expected = vec![vec![], vec![0]]
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>();

        assert_eq!(indicies, expected);
    }
}
