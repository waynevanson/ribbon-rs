pub mod tag;
use js_sys::Array;
use ribbon::{index_dimensional::IndexDimensional, prelude::*};
use std::collections::HashMap;
use tag::HtmlTag;
use wasm_bindgen::JsValue;
use web_sys::{console, Document, Element};

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

fn find_closest_parent<A>(
    index: IndexDimensional,
    nodes: &HashMap<IndexDimensional, A>,
) -> Option<&A> {
    index
        .parents()
        .map(|parents| parents.into_iter().rev().collect::<Vec<_>>())
        .and_then(|parents| {
            let mut result: Option<&A> = None;

            for index in &parents {
                if let Some(node) = nodes.get(index) {
                    result = Some(node);
                    break;
                };
            }

            result
        })
}

impl HtmlRender {
    pub fn new(document: Document, element: Element) -> Self {
        HtmlRender {
            document,
            element,
            nodes_by_vnode_index: Default::default(),
        }
    }

    pub fn paint(&mut self, vnode: VNode) -> () {
        vnode.into_iter().for_each(|(index, vnode)| {
            let parent = find_closest_parent(index.clone(), &self.nodes_by_vnode_index)
                .unwrap_or(&self.element);

            let node: web_sys::Node = match vnode {
                VNode::Text { value } => self.document.create_text_node(value).into(),
                VNode::Element { tag, .. } => self
                    .document
                    .create_element(&tag.to_string())
                    .unwrap()
                    .into(),
            };

            console::log_1(&JsValue::from_str(&format!("{:?}", index.clone())));

            console::log_1(&JsValue::from_str(&format!(
                "{:?}",
                self.nodes_by_vnode_index.clone()
            )));

            parent.append_child(&node).unwrap();

            self.nodes_by_vnode_index.insert(index.clone(), node);
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ribbon::index_dimensional::IndexDimensional;

    use crate::find_closest_parent;

    #[test]
    fn should_lookup_right_element() {
        let nodes: HashMap<IndexDimensional, isize> = HashMap::from([(vec![].into(), 1)]);

        let result = find_closest_parent(vec![1, 3, 8, 4].into(), &nodes);
        assert_eq!(result, Some(&1));
    }
}
