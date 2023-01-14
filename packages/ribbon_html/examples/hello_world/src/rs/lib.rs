#![allow(non_upper_case_globals)] // wasm_bindgen lint fix

use ribbon::prelude::*;
use ribbon_html::{tag::HtmlTag, HtmlRender, Node};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

#[derive(Debug, Default)]
struct HelloWorld;

impl Stateless<Node> for HelloWorld {
    fn view(&self) -> Node {
        Node::Element {
            tag: HtmlTag::Div,
            attrs: Default::default(),
            children: vec![Node::text("Hello, World!")],
        }
        .into()
    }
}

#[wasm_bindgen]
pub fn main() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let mut renderer = HtmlRender::new(document, body.into());
    let vnode = HelloWorld.view().into();

    renderer.paint(vnode);
}
