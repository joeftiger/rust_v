use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

#[derive(Default)]
pub struct SortedVec<T: Ord> {
    vec: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub const fn new() -> Self {
        Self {
            vec: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
        }
    }

    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        Self {
            vec: Vec::from_raw_parts(ptr, length, capacity)
        }
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.vec.reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit()
    }

    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.vec.into_boxed_slice()
    }

    pub fn truncate(&mut self, len: usize) {
        self.vec.truncate(len)
    }

    pub fn as_slice(&self) -> &[T] {
        self.vec.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.vec.as_mut_slice()
    }

    pub fn as_ptr(&self) -> *const T {
        self.vec.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.vec.as_mut_ptr()
    }

    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.vec.set_len(new_len)
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.vec.remove(index)
    }

    pub fn retain<F: FnMut(&T) -> bool>(&mut self, f: F) {
        self.vec.retain(f)
    }

    pub fn dedup_by_key<F: FnMut(&mut T) -> K, K: PartialEq>(&mut self, key: F) {
        self.vec.dedup_by_key(key)
    }

    pub fn dedup_by<F: FnMut(&mut T, &mut T) -> bool>(&mut self, same_bucket: F) {
        self.vec.dedup_by(same_bucket)
    }

    pub fn push(&mut self, value: T) {
        let len = self.vec.len();

        for index in 0..len {
            if value.eq(&self.vec[index]) {
                panic!()
            }

            if value.lt(&self.vec[index]) {
                self.vec.insert(index, value);
                return;
            }
        }

        self.vec.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    pub fn append(&mut self, other: &mut Self) {
        self.vec.append(&mut other.vec);
        self.vec.sort();
    }

    // TODO: Fix import of Drain<T>
    // pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<T> {
    //     self.vec.drain(range)
    // }

    pub fn clear(&mut self) {
        self.vec.clear()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    #[must_use = "use `.truncate()` if you don't need the other half"]
    pub fn split_off(&mut self, at: usize) -> Self {
        Self {
            vec: self.vec.split_off(at)
        }
    }

    pub fn resize_with<F: FnMut() -> T>(&mut self, new_len: usize, f: F) {
        self.vec.resize_with(new_len, f);
        self.vec.sort()
    }


    // TODO: Fix import of Splice<I::IntoIter>
    // pub fn splice<R: RangeBounds<usize>, I: IntoIterator<Item = T>>(&mut self, range: R, replace_with: I) -> Splice<I::IntoIter> {
    //     let splice = self.vec.splice(range, replace_with);
    //     self.vec.sort();
    //
    //     splice
    // }

    /// Returns the index of the given value (if found)
    /// O(log2(n))
    pub fn index_of(&self, value: T) -> Option<usize> {
        let mut left = 0;
        let mut right = self.len() - 1;

        while left <= right {
            let middle = f32::floor((left + right) as f32 / 2.0) as usize;

            if self[middle] < value {
                left = middle + 1;
            } else if self[middle] > value {
                right = middle - 1;
            } else {
                return Some(middle);
            }
        }

        None
    }

    /// Returns the closest next lower index of the given value (if not empty).
    /// O(log2(n))
    pub fn index_of_next_lower(&self, value: T) -> Option<usize> {
        let mut left = 0;
        let mut right = self.len() - 1;

        if self.len() == 1 {
            return Some(0);
        }

        while left <= right {
            let middle = f32::floor((left + right) as f32 / 2.0) as usize;

            if self[middle] > value {
                right = middle - 1;
            } else if middle > 0 && self[middle - 1] < value {
                left = middle + 1;
            } else {
                Some(middle);
            }
        }

        None
    }

    /// Returns the closest next upper index of the given value (if not empty).
    /// O(log2(n))
    pub fn index_of_next_upper(&self, value: T) -> Option<usize> {
        let mut left = 0;
        let mut right = self.len() - 1;

        if self.len() == 1 {
            return Some(0);
        }

        while left <= right {
            let middle = f32::floor((left + right) as f32 / 2.0) as usize;

            if self[middle] < value {
                left = middle + 1;
            } else if middle < self.len() - 1 && self[middle + 1] > value {
                right = middle - 1;
            } else {
                Some(middle);
            }
        }

        None
    }
}

impl<T: Ord + Clone> SortedVec<T> {
    pub fn resize(&mut self, new_len: usize, value: T) {
        self.vec.resize(new_len, value);
        self.vec.sort();
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        self.vec.extend_from_slice(other);
    }
}

impl<T: Ord, I: SliceIndex<[T]>> Index<I> for SortedVec<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T: Ord, I: SliceIndex<[T]>> IndexMut<I> for SortedVec<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl<T: Ord> From<Vec<T>> for SortedVec<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut sorted_vec = Self {
            vec
        };
        sorted_vec.vec.sort();

        sorted_vec
    }
}
