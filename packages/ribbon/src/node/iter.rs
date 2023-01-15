use super::VNode;
use crate::index_dimensional::IndexDimensional;

pub struct DepthIterWithIndex<'a> {
    node: &'a VNode,
    index: IndexDimensional,
}

impl<'a> Iterator for DepthIterWithIndex<'a> {
    type Item = (IndexDimensional, &'a VNode);

    // gotta go back up to node that has children at next index that is not self.
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.clone();
        let vnode = self.node.nth(&index)?;
        let current = (index, vnode);

        let deep = self.index.clone().increment_depth();
        let step = || self.index.clone().increment_step();

        self.index = self.node.nth(&deep).map(|_| deep).or_else(|| {
            let index = step()?;
            self.node.nth(&index)?;
            index.into()
        })?;

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
        let one = VNode::Element {
            attributes: Default::default(),
            tag: "".to_string(),
            children: vec![VNode::text("")],
        };
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

        let expected = vec![vec![], vec![0], vec![0, 0], vec![1], vec![1, 0], vec![1, 1]]
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>();

        assert_eq!(indicies, expected);
    }
}
