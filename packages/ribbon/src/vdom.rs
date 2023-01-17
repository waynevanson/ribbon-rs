use std::vec;

use crate::vnode::VNode;
use graph::prelude::*;

pub struct VDom {
    root: VNode,
}

impl<'a> IntoIterator for &'a VDom {
    type Item = &'a VNode;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            root: &self.root,
            visiting: None,
            parents: self
                .root
                .children()
                .filter(|vnodes| !vnodes.is_empty())
                .map(|_| vec![(&self.root, 0)])
                .unwrap_or_default(),
        }
    }
}

impl Graph<usize> for VDom {
    fn edge_count(&self) -> usize {
        self.root.edge_count()
    }

    fn node_count(&self) -> usize {
        self.root.node_count()
    }
}

pub type Breadth = usize;

pub struct Iter<'a> {
    root: &'a VNode,
    visiting: Option<&'a VNode>,
    parents: Vec<(&'a VNode, Breadth)>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a VNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.visiting = self
            .visiting
            .and_then(|visited| {
                visited.child(0).or_else(|| {
                    let &(parent, breadth) = self.parents.last()?;

                    let child = parent
                        .child(breadth)
                        .expect("Child should always exists when the parent exists");

                    let count = child
                        .children()
                        .map(|nodes| nodes.len())
                        .filter(|&len| len > 0)
                        .map(|len| len - 1)
                        .unwrap_or_default();

                    if let true = breadth == count {
                        self.parents
                            .pop()
                            .expect("Element should be removed when sending it on it's way");
                    };

                    Some(child)
                })
            })
            .or(self.root.into());

        self.visiting
    }
}
