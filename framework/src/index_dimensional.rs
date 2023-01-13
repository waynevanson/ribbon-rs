use std::slice::Iter;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct IndexDimensional {
    inner: Vec<usize>,
}

impl IndexDimensional {
    pub fn increment_step_mut(&mut self) -> Option<&Self> {
        self.inner.last_mut().map(|last| *last += 1);
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
