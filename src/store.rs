use std::ops::{Index, IndexMut};
use std::slice::Iter;

pub struct Store<T>(Vec<T>);

impl<T> Store<T> {
    pub fn push(&mut self, obj: T) -> usize {
        self.0.push(obj);
        self.0.len() - 1
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Index<usize> for Store<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Store<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
