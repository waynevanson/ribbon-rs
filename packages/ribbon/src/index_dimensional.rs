use std::slice::Iter;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndexDimensional {
    inner: Vec<usize>,
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

    pub fn iter<'a>(&'a self) -> Iter<'a, usize> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a IndexDimensional {
    type Item = &'a usize;
    type IntoIter = Iter<'a, usize>;

    fn into_iter(self) -> Iter<'a, usize> {
        self.inner.iter()
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
}
