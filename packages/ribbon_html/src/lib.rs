struct WebEnv;

mod components {
    use ribbon::{
        child::Child,
        environment::Environment,
        props::Props,
        view::{Dispatcher, ToNode, View},
    };

    use crate::WebEnv;

    // div has optional params.
    // How should we handle these?
    // props can use a buildler for DivProps
    struct DivProps {}
    struct Div {}

    impl Props for Div {
        type Props = DivProps;
    }

    impl Environment for Div {
        type Environment = WebEnv;
    }

    impl View for Div {
        type Node = HtmlFakeNode;

        // self dispatch?
        fn view(&self, dispatcher: Dispatcher<Self>) -> Self::Node {
            HtmlFakeNode
        }
    }

    impl Child for Div {}

    struct HtmlFakeNode;

    impl ToNode for HtmlFakeNode {
        type Environment = WebEnv;
    }
}
