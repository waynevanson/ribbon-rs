pub trait ToChildren
where
    Self: Sized,
{
    fn to_children(&self) -> Option<&[Self]>;

    fn nth_child(&self, index: usize) -> Option<&Self> {
        self.to_children()?.iter().nth(index)
    }
}

pub type Breadth = usize;

pub struct DepthFirstSearchIter<'a, A> {
    root: &'a A,
    depths: Vec<(&'a A, Breadth)>,
}

// go to current by getting to the deepest

impl<'a, A> Iterator for DepthFirstSearchIter<'a, A>
where
    A: ToChildren,
{
    type Item = &'a A;

    //add depth to the place
    //
    // go back to parent and look into next child
    // repeat until at parent.
    // when last of indexmap returns none, we are at parent.
    // and done
    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .depths
            .last()
            .map(|value| value.0)
            .unwrap_or(&self.root);

        next.nth_child(0)
            .map(|child| self.depths.push((child, 0)))
            .or_else(|| {
                self.depths
                    .iter()
                    .map(|value| *value)
                    .enumerate()
                    .rfold(None, |accumulator, (index, (parent, breadth))| {
                        accumulator.or(parent.nth_child(breadth).map(|_| index))
                    })
                    .map(|index| self.depths.truncate(index))
            })?;

        Some(next)
    }
}
