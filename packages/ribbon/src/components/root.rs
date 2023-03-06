use super::{context::Context, props::Props, references::References, state::State};

pub trait Root<S, A, R>
where
    Self: Props<Props = ()>,
    Self: Context<Context = ()>,
    Self: State<State = S>,
    Self: References<Action = A, References = R>,
{
}
