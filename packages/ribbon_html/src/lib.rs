pub mod tag;
use ribbon::{index_dimensional::IndexDimensional, prelude::*};
use std::collections::HashMap;
use tag::HtmlTag;
use web_sys::{Document, Element};

#[derive(Clone)]
pub struct HtmlNode {
    pub tag: HtmlTag,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Self>,
}

impl HtmlNode {
    pub fn div() -> Self {
        HtmlNode {
            tag: HtmlTag::Div,
            attrs: Default::default(),
            children: Default::default(),
        }
    }

    pub fn attrs<T>(&mut self, attributes: T) -> &Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        for (property, value) in attributes {
            self.attrs.insert(property, value);
        }

        self
    }
}

impl From<HtmlNode> for VNode {
    fn from(html_node: HtmlNode) -> Self {
        Self::Element {
            tag: html_node.tag.to_string(),
            attributes: html_node.attrs,
            children: html_node.children.into_iter().map(|x| x.into()).collect(),
        }
    }
}

pub struct HtmlRender {
    pub document: Document,
    pub element: Element,
}

impl HtmlRender {
    pub fn new(document: Document, element: Element) -> Self {
        HtmlRender { document, element }
    }

    pub fn paint(&mut self, vnode: VNode) -> () {
        let mut element_by_vnode_index: HashMap<IndexDimensional, Element> = Default::default();

        // transform the vnode to a html element
        // keep track of current parent, and a hashmap of Index to HtmlNode values
        //
        // For each vnode, we check to see how we can attach it to the body
        // If VNODE::Element, create a HTML element and append it to the parent.
        // if VNODE::Text, set the parents' children to this text.
        //
        // main problem is how to effectively find the correct parent element.
        // always available.
        // look up closest parent from currently mounted maps
        for (index, vnode) in vnode.iter_with_index() {
            let parent = element_by_vnode_index.get(&index).unwrap_or(&self.element);

            match vnode {
                VNode::Text { value } => {
                    parent.set_inner_html(value);
                }
                VNode::Element {
                    tag,
                    attributes,
                    children: _,
                } => {
                    let element = self.document.create_element(tag).unwrap();

                    for (name, value) in attributes {
                        element.set_attribute(name, value).unwrap();
                    }

                    parent.append_child(&element).unwrap();

                    element_by_vnode_index.insert(index, element);
                }
            };
        }

        // parent already in body, no need to inject it.
    }
}
