use super::view::View;

/// This trait is used as a placeholder for an impl that contains a negative trait bound to props.
///
/// `!Props`
///
/// Only structures that have can be rendered.
pub trait Child: View {}
