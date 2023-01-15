use std::slice::{Iter, IterMut};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndexDimensional {
    pub inner: Vec<usize>,
}

impl IndexDimensional {
    pub fn increment_step_mut(&mut self) -> Option<&Self> {
        self.inner.last_mut().map(|last| *last += 1)?;
        Some(self)
    }

    pub fn increment_step(mut self) -> Option<Self> {
        self.increment_step_mut().cloned()
    }

    pub fn increment_depth_mut(&mut self) -> &Self {
        self.inner.push(0);
        self
    }

    pub fn increment_depth(mut self) -> Self {
        self.increment_depth_mut().clone()
    }

    pub fn decrement_step_mut(&mut self) -> Option<&Self> {
        self.inner
            .last_mut()
            .filter(|last| **last > 0)
            .map(|last| *last -= 1)?;

        Some(self)
    }

    pub fn decrement_depth_mut(&mut self) -> Option<&Self> {
        let len = self.inner.len();
        if len == 0 {
            None
        } else {
            self.inner.truncate(len - 1);
            Some(self)
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, usize> {
        self.into_iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, usize> {
        self.into_iter()
    }

    // None it has no parents.
    pub fn parents(mut self) -> Option<Vec<Self>> {
        let x = self.decrement_depth_mut()?;
        let q = x
            .clone()
            .inner
            .iter()
            .try_fold(vec![x.clone()], |mut acc, _| {
                let index = acc
                    .last()
                    .cloned()
                    .and_then(|mut index| index.decrement_depth_mut().cloned());

                index.map(|index| {
                    acc.push(index);
                    acc
                })
            })?;

        q.into_iter().rev().collect::<Vec<Self>>().into()
    }
}

impl From<Vec<usize>> for IndexDimensional {
    fn from(inner: Vec<usize>) -> Self {
        IndexDimensional { inner }
    }
}

impl<'a> IntoIterator for &'a IndexDimensional {
    type Item = &'a usize;
    type IntoIter = Iter<'a, usize>;

    fn into_iter(self) -> Iter<'a, usize> {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut IndexDimensional {
    type Item = &'a mut usize;
    type IntoIter = IterMut<'a, usize>;

    fn into_iter(self) -> IterMut<'a, usize> {
        self.inner.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod increment_step {
        use super::*;

        #[test]
        fn should_return_some_and_increment_existing_last_value() {
            let result = IndexDimensional {
                inner: vec![1, 2, 3, 4, 5, 6],
            }
            .increment_step();

            let expected = Some(IndexDimensional {
                inner: vec![1, 2, 3, 4, 5, 7],
            });

            assert_eq!(result, expected);
        }

        #[test]
        fn should_return_none_when_empty() {
            let result = IndexDimensional { inner: vec![] }.increment_step();

            let expected = None;

            assert_eq!(result, expected);
        }
    }

    mod increment_step_mut {
        use super::*;

        #[test]
        fn should_return_some_and_increment_existing_last_value() {
            let mut data = IndexDimensional {
                inner: vec![1, 2, 3, 4, 5, 6],
            };

            let result = data.increment_step_mut();

            let next = IndexDimensional {
                inner: vec![1, 2, 3, 4, 5, 7],
            };

            let expected = Some(&next);

            assert_eq!(result, expected);
        }

        #[test]
        fn should_return_none_when_empty() {
            let mut data = IndexDimensional { inner: vec![] };
            let result = data.increment_step_mut();

            let expected = None;

            assert_eq!(result, expected);
        }
    }

    mod parents {
        use super::*;

        #[test]
        fn should_return_none_when_node_is_root() {
            let index: IndexDimensional = vec![].into();
            let result = index.parents();
            let expected = None;

            assert_eq!(result, expected);
        }

        #[test]
        fn should_return_some_when_node_is_child() {
            let index: IndexDimensional = vec![4, 0, 8, 3, 4].into();
            let result = index.parents();
            let expected = Some(
                vec![
                    vec![].into(),
                    vec![4].into(),
                    vec![4, 0].into(),
                    vec![4, 0, 8].into(),
                    vec![4, 0, 8, 3].into(),
                ]
                .into(),
            );

            assert_eq!(result, expected);
        }
    }
}
