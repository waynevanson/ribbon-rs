## Goal

Use traits and trait macros to create components.

## Why

I've been using async-graphql and it feels good.
I want the type safety of rust without it feeling too foreign.
Also some serde support seems nice too. Transform from rust into the component layout?
rust into component tree,
Could be isomorphic -> Work without VDOM anywhere.

## Examples (Possible)

```rs
#[Stateful]
struct Counter {
    count: isize
}

struct Props {
    initial: isize
}

enum Message {
    Increment,
    Decrement,
}

#[Component]
impl Counter {
    type Props = Props;
    type Message = Message;

    fn create(props: Self::Props) -> Self {
        Counter { count: props.initial }
    }

    fn view(&self, environment: Env) -> Rendered {
        (
            Env::Div,
            [("attr-1", "hello")],
            []
        ).into()
    }

    fn update(&mut self, message: Self::Message) -> Self {
        match message {
            Increment => self.count += 1,
            Decrement => self.count -= 1,
        }
    }
}

#[Stateless]
struct Button;

#[Component]
impl Button {
    fn view(&self, environment: Env) -> Rendered {
        (
            Env::Button,
            [],
            [self.text]
        ).into()
    }
}



fn main() {
    let app = app();
    app.render()
}
```

## Notes

Could do something complicated like we do for serde. Ser and De.

FromNode ToNode, don't think so.

Data model

Text
Element
Portal?
Fragment
