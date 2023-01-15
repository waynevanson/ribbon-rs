#![allow(non_upper_case_globals)] // wasm_bindgen lint fix

use ribbon::prelude::*;
use ribbon_html::{tag::HtmlTag, HtmlRender, Node};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

#[derive(Debug, Default)]
struct Counter {
    count: isize,
}

impl Counter {
    fn increment(mut self) -> Self {
        self.count += 1;
        self
    }

    fn decrement(mut self) -> Self {
        self.count -= 1;
        self
    }
}

enum Message {
    Increment,
    Decrement,
}

impl Stateful<Node> for Counter {
    type Message = Message;

    fn view(&self) -> Node {
        let increment = Node::Element {
            tag: HtmlTag::Button,
            attrs: Default::default(),
            children: vec![Node::text("increment")],
        };

        let decrement = Node::Element {
            tag: HtmlTag::Button,
            attrs: Default::default(),
            children: vec![Node::text("decrement")],
        };

        let display = Node::text(format!("Counter: {}", self.count));

        let container = Node::Element {
            tag: HtmlTag::Div,
            attrs: Default::default(),
            children: vec![increment, decrement],
        };

        container
    }

    fn update(self, message: Self::Message) -> Option<Self> {
        match message {
            Message::Increment => self.increment(),
            Message::Decrement => self.decrement(),
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
    let vnode = Counter { count: 0 }.view().into();

    renderer.paint(vnode);
}
