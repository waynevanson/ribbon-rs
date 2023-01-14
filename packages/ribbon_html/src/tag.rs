use strum::Display;

#[derive(Clone, Display, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum HtmlTag {
    Div,
}
