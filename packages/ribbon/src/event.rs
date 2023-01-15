// hard basket, interesting.
pub struct Event {
    name: String,
    handle: Box<dyn Fn() -> Box<()>>,
}
