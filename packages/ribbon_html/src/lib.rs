pub mod tag;
use ribbon::prelude::*;
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
}

impl HtmlRender {
    pub fn new(document: Document, element: Element) -> Self {
        HtmlRender { document, element }
    }

    pub fn paint(&mut self, vnode: VNode) -> () {
        // add a graph to this
    }
}
