use std::iter::FusedIterator;

pub trait ToChildren
where
    Self: Sized,
{
    fn to_children(&self) -> Option<&[Self]>;

    fn nth_child(&self, index: usize) -> Option<&Self> {
        self.to_children()?.iter().nth(index)
    }
}

type Breadth = usize;

pub struct DepthFirstSearchIter<'a, A> {
    root: &'a A,
    // parents that are in the middle of iterating children, including root
    // does not include parents that are at the last child.
    unfinished: Vec<(&'a A, Breadth)>,
}

// go to current by getting to the deepest

enum Message<'a, A> {
    Depth { value: &'a A },
    Breadth,
}

impl<'a, A> Iterator for DepthFirstSearchIter<'a, A>
where
    A: ToChildren,
{
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .unfinished
            .last()
            .map(|value| value.0)
            .unwrap_or_else(|| {
                self.unfinished.push((&self.root, 0));
                &self.root
            });

        let message = next
            .nth_child(0)
            // if we can go deeper, add that node to list of unfinished.
            .map(|child| Message::Depth { value: child })
            .unwrap_or(Message::Breadth);

        match message {
            Message::Depth { value } => {
                self.unfinished.push((&value, 0));
            }
            Message::Breadth => {
                self.unfinished.truncate(self.unfinished.len() - 1);
                let (node, breadth) = self.unfinished.last_mut().unwrap();
                let len = node.to_children().unwrap().len();

                let is_last = *breadth == len - 1;

                if is_last {
                    self.unfinished.truncate(self.unfinished.len() - 1);
                } else {
                    *breadth += 1;
                }
            }
        }

        Some(next)
    }
}

impl<'a, A> FusedIterator for DepthFirstSearchIter<'a, A> where A: ToChildren {}
