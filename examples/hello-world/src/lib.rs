use rust_web_framework::{
    html::{HtmlNode, HtmlRender},
    prelude::*,
};
use wasm_bindgen::prelude::*;
use web_sys::{window, Element};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    struct Counter {
        count: usize,
    }

    impl Stateless<HtmlNode> for Counter {
        fn view(&self) -> HtmlNode {
            HtmlNode::div()
                .attrs([("count".to_string(), self.count.to_string())])
                .to_owned()
        }
    }

    let counter = Counter { count: 5 };

    let root: VNode = counter.view().into();

    let window = window().ok_or(JsValue::from_str("window"))?;
    let document = window.document().ok_or(JsValue::from_str("document"))?;

    let body = document.body();
    let element: Element = body.unwrap().into();

    HtmlRender::new(document, element).paint(root);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn render() {
        struct Counter {
            count: usize,
        }

        impl Stateless<HtmlNode> for Counter {
            fn view(&self) -> HtmlNode {
                HtmlNode::div()
                    .attrs([("count".to_string(), self.count.to_string())])
                    .to_owned()
            }
        }

        let counter = Counter { count: 5 };

        let _root: VNode = counter.view().into();
    }
}
