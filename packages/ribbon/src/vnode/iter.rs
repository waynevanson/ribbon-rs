use crate::vnode::VNode;
use std::vec;

impl<'a> IntoIterator for &'a VNode {
    type Item = &'a VNode;
    type IntoIter = IterValue<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterValue {
            root: &self,
            visiting: None,
            parents: Some(self.children())
                .filter(|children| !children.is_empty())
                .map(|_| vec![(self, 0)])
                .unwrap_or_default(),
        }
    }
}

pub type Breadth = usize;

pub struct IterValue<'a> {
    root: &'a VNode,
    visiting: Option<&'a VNode>,
    parents: Vec<(&'a VNode, Breadth)>,
}

pub type Edge = (usize, usize);

impl<'a> Iterator for IterValue<'a> {
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

                    let count = Some(child.children())
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
