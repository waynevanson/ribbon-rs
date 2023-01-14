use super::VNode;
use crate::index_dimensional::IndexDimensional;

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

impl<'a> IntoIterator for &'a VNode {
    type Item = (IndexDimensional, &'a VNode);
    type IntoIter = DepthIterWithIndex<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DepthIterWithIndex {
            node: self,
            index: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_iterate_values_in_order() {
        let one = VNode::text("one");
        let four = VNode::text("four");
        let three = VNode::Element {
            tag: "three".to_string(),
            attributes: Default::default(),
            children: vec![],
        };
        let two = VNode::element("tag", [("c".to_string(), "d".to_string())], [three, four]);
        let zero = VNode::element(
            "first",
            [("one".to_string(), "two".to_string())],
            [one, two],
        );

        let indicies = zero
            .iter_with_index()
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let expected = vec![vec![], vec![0], vec![1], vec![1, 0], vec![1, 1]]
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>();

        assert_eq!(indicies, expected);
    }
}
