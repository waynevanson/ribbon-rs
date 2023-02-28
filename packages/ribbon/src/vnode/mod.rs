/// `VNode`'s hold all the information need for a render.
/// This includes attributes, callbacks and refs.
use graph::prelude::*;
use std::collections::HashMap;

// I don't think we want to care about size becuase it makes our lives more difficult.
enum Attribute {
    Callback(Box<dyn FnOnce()>),
    // I mean this thing can be any kind of value
    String(String),
    Number(usize),
    Boolean(bool),
}
pub enum VNode {
    Text {
        value: String,
    },
    Element {
        tag: String,
        attributes: HashMap<String, Attribute>,
        children: Vec<VNode>,
    },
    Fragment {
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

pub type VNodeGraph<'a> = DirectedCsrGraph<usize, &'a VNode, ()>;

// plan is to use these to perform diffs.
impl<'a> From<&'a VNode> for VNodeGraph<'a> {
    fn from(vnode: &'a VNode) -> Self {
        let (edges, nodes) = vnode.for_graph();

        GraphBuilder::new()
            .csr_layout(CsrLayout::Deduplicated)
            .edges(edges)
            .node_values(nodes)
            .build()
    }
}

impl VNode {
    pub fn tag(&self) -> Option<&str> {
        match self {
            VNode::Element { tag, .. } => Some(tag),
            _ => None,
        }
    }

    pub fn children(&self) -> &Vec<VNode> {
        match &self {
            Self::Text { value: _ } => &Vec::new(),
            Self::Element { children, .. } => children,
        }
    }

    pub fn text(str: &str) -> Self {
        Self::Text {
            value: str.to_string(),
        }
    }

    pub fn fragment(children: Vec<VNode>) -> Self {
        Self::Fragment { children }
    }
}

pub trait ToVNode {
    fn to_vnode(self) -> VNode;
}

impl ToVNode for VNode {
    fn to_vnode(self) -> VNode {
        self
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
