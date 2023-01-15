use strum::Display;

#[derive(Clone, Display, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum HtmlTag {
    Div,
    Button,
}
