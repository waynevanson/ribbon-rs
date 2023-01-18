mod iter;
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

// plan is to use these to perform diffs and
impl<'a> From<&'a VNode> for DirectedCsrGraph<usize, &'a VNode, ()> {
    fn from(vnode: &'a VNode) -> Self {
        let mut current_node = 0;
        let mut visited: Option<(&'a VNode, usize)> = None;

        let mut parents = if vnode.children().is_empty() {
            vec![(vnode, 0, current_node)]
        } else {
            vec![]
        };

        let mut edges = vec![];
        let mut nodes = vec![];

        while !parents.is_empty() {
            if let Some((current, node)) = visited {
                let child = current.child(0);

                if let Some(child) = child {
                    // exists, add edge and node
                    edges.push((node, current_node));
                    nodes.push(child);
                    parents.push((current, 0, node));
                } else {
                }
            } else {
                visited = Some((vnode, 0));
            }

            current_node += 1;
        }

        GraphBuilder::new()
            .csr_layout(CsrLayout::Sorted)
            .edges(edges)
            .node_values(nodes)
            .build()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_one_element_in_childless_graph() {
        let vnode = VNode::text("zero");
        let graph = DirectedCsrGraph::from(&vnode);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);
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
