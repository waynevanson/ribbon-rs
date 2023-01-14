pub mod tag;
use ribbon::{index_dimensional::IndexDimensional, prelude::*};
use std::collections::HashMap;
use tag::HtmlTag;
use web_sys::{Document, Element};

pub enum Node {
    Text(String),
    Element {
        tag: HtmlTag,
        attrs: HashMap<String, String>,
        children: Vec<Self>,
    },
}

impl Node {
    pub fn text<S: ToString>(test: S) -> Self {
        Self::Text(test.to_string())
    }

    pub fn div() -> Self {
        Node::Element {
            tag: HtmlTag::Div,
            attrs: Default::default(),
            children: Default::default(),
        }
    }
}

impl From<Node> for VNode {
    fn from(node: Node) -> Self {
        match node {
            Node::Element {
                tag,
                attrs,
                children,
            } => Self::Element {
                tag: tag.to_string(),
                attributes: attrs,
                children: children.into_iter().map(|x| x.into()).collect(),
            },
            Node::Text(value) => Self::Text { value },
        }
    }
}

pub struct HtmlRender {
    document: Document,
    element: Element,
    nodes_by_vnode_index: HashMap<IndexDimensional, web_sys::Node>,
}

impl HtmlRender {
    pub fn new(document: Document, element: Element) -> Self {
        HtmlRender {
            document,
            element,
            nodes_by_vnode_index: Default::default(),
        }
    }

    fn find_closest_parent(&self, index: IndexDimensional) -> &web_sys::Node {
        index
            .parents()
            .map(|parents| parents.into_iter().rev().collect::<Vec<_>>())
            .and_then(|parents| {
                parents.iter().fold(None, |acc, index| {
                    acc.or(self.nodes_by_vnode_index.get(index))
                })
            })
            .unwrap_or(&self.element)
    }

    pub fn paint(&mut self, vnode: VNode) -> () {
        vnode.into_iter().for_each(|(index, vnode)| {
            let parent = self.find_closest_parent(index.clone());

            let node: web_sys::Node = match vnode {
                VNode::Text { value } => self.document.create_text_node(value).into(),
                VNode::Element { tag, .. } => self
                    .document
                    .create_element(&tag.to_string())
                    .unwrap()
                    .into(),
            };

            parent.append_child(&node).unwrap();

            self.nodes_by_vnode_index.insert(index.clone(), node);
        });
    }
}
