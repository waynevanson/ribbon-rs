use graph::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
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
    fn for_graph(&self) -> (Vec<(usize, usize)>, Vec<&Self>) {
        if self.children().len() == 0 {
            (vec![], vec![self])
        } else {
            let mut visited_vnode = self;
            let mut visited_node = 0;

            let mut parents = vec![];

            let mut edges = vec![];
            let mut nodes = vec![self];

            // currently assumes it will have more than one child,
            // makes aprent always
            while !parents.is_empty() || visited_node == 0 {
                let child_node = visited_node + 1;

                let child_vnode = if let Some(child_vnode) = visited_vnode.children().first() {
                    nodes.push(child_vnode);
                    edges.push((visited_node, child_node));

                    // could check if parent has kids before adding it to the list of stuff
                    let parent_vnode = visited_vnode;
                    let parent_node = visited_node;
                    let parent_breadth = 1;

                    if let true = parent_vnode.child(parent_breadth + 1).is_some() {
                        parents.push((parent_vnode, parent_node, parent_breadth));
                    };

                    child_vnode
                } else {
                    let &(parent_vnode, parent_node, parent_breadth) = parents
                        .last()
                        .expect("Deepest parent always exists in a loop where it is not empty");

                    let child_vnode = parent_vnode
                        .child(parent_breadth)
                        .expect("Child should always exist when it comes from a parent");

                    nodes.push(child_vnode);
                    edges.push((parent_node, child_node));

                    if parent_vnode.child(parent_breadth + 1).is_some() {
                        parents
                            .last_mut()
                            .expect("Should exist as the parent should exist here")
                            .2 += 1;
                    } else {
                        parents
                            .pop()
                            .expect("Should be poppable as the parent should exist here");
                    }

                    child_vnode
                };

                visited_vnode = child_vnode;
                visited_node = child_node;
            }

            (edges, nodes)
        }
    }
}

// plan is to use these to perform diffs and
impl<'a> From<&'a VNode> for DirectedCsrGraph<usize, &'a VNode, ()> {
    fn from(vnode: &'a VNode) -> Self {
        let (edges, nodes) = vnode.for_graph();

        GraphBuilder::new()
            .csr_layout(CsrLayout::Sorted)
            .edges(edges)
            .node_values(nodes)
            .build()
    }
}

impl VNode {
    pub fn children(&self) -> &[Self] {
        match self {
            Self::Text { value: _ } => [].as_slice(),
            Self::Element { children, .. } => children.as_slice(),
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

    pub fn child(&self, indice: usize) -> Option<&Self> {
        match self {
            VNode::Text { value: _ } => None,
            VNode::Element { children, .. } => children.iter().nth(indice),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]

    fn should_return_two_elements_in_graph_containing_child_() {
        let one = VNode::text("one");
        let vnode = VNode::Element {
            tag: "zero".to_string(),
            attributes: Default::default(),
            children: vec![one.clone()],
        };
        let result = vnode.for_graph();
        assert_eq!(result, (vec![(0, 1)], vec![&vnode, &one]));
    }

    #[test]
    fn should_return_one_element_in_childless_graph() {
        let vnode = VNode::text("zero");
        let result = vnode.for_graph();
        assert_eq!(result, (vec![], vec![&vnode]))
    }

    #[test]
    fn should_return_two_elements_in_graph_containing_child() {
        let vnode = VNode::Element {
            tag: "".to_string(),
            attributes: Default::default(),
            children: vec![VNode::text("zero")],
        };
        let graph = DirectedCsrGraph::from(&vnode);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }
}
