use std::{fmt::Debug, ops::{Index, IndexMut}};

mod test;

/// Make a vector that can rotate without changing place in memory.
/// This reduces the computational order from O(N) to O(1) for all
/// shifting operations.
#[derive(Clone)]
pub struct VecRotate<T: Clone> {
    array: Vec<T>, // could put in an array instead
    start_index: usize,
    length: usize,
}

impl<T: Clone> VecRotate<T> {

    pub fn new(array: Vec<T>) -> Self {
        let length = array.len();
        VecRotate {
            array: array,
            start_index: 0,
            length: length,
        }
    }

    /// Check if the vector is empty.
    pub fn is_empty(&self) -> bool {
        if self.length == 0 {
            true
        } else {
            false
        }
    }
    
    // TODO: Check that no off by one errors are present with using self.length
    /// Shift all the elements forward by 'steps' places.
    /// If the array is empty, don't do anything.
    pub fn shift_backward(&mut self, steps: usize) -> () {
        if !self.is_empty() {
            let shift_step = steps % self.length;
            let idx = self.start_index + shift_step;
            if idx < self.length {
                self.start_index = idx;
            } else {
                self.start_index = idx - self.length;
            }
        }
    }

    pub fn shift_forward(&mut self, steps: usize) -> () {
        if !self.is_empty() {
            let shift_step = steps % self.length;
            if shift_step > self.start_index {
                self.start_index = self.length - (shift_step - self.start_index)
            } else {
                self.start_index -= shift_step;
            }
        }
    }

    pub fn push(&mut self, element: T) {
        self.array.insert(self.start_index, element);
        self.length += 1;
        self.start_index += 1;
    }
    
    pub fn extend(&mut self, extension: &[T]) {
        let num_new = extension.len();
        self.array = [
            &self.array[..self.start_index],
            extension,
            &self.array[self.start_index..]
        ].concat();
        self.length += num_new;
        self.start_index += num_new;
    }

    pub fn index_via_array(&self, index: &[usize]) -> Vec<T> {
        index.iter().map(|&idx| &self[idx]).cloned().collect()
    }

    /// Replace the rotated array entries at the specified indexx with new entries.
    pub fn update_via_array(&mut self, index: &[usize], new_entries: &[T]) {
        for (i, entry) in new_entries.iter().enumerate() {
            self[index[i]] = entry.clone();
        }
    } 

    fn wrap_index(&self, index: usize) -> usize 
    {
        let tail = self.length - self.start_index;
        if index < tail {
                self.start_index + index
        } else {
            index - tail
        }
    }
}

impl<T: Clone> IntoIterator for VecRotate<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array[self.start_index..].iter()
            .chain(self.array[..self.start_index].iter())
            .cloned().collect::<Vec<T>>().into_iter()
    }
}

impl<T: Clone> From<&[T]> for VecRotate<T> {

    fn from(value: &[T]) -> Self {
        Self {
            array: value.clone().to_vec(),
            start_index: 0,
            length: value.len(),
        }
    }
}

impl<T: Clone, const N: usize> From<&[T; N]> for VecRotate<T> {
    fn from(value: &[T; N]) -> Self {
        Self {
            array: value.to_vec(),
            start_index: 0,
            length: value.len(),
        }
    }
}

impl<T: Clone> Into<Vec<T>> for VecRotate<T> {
    fn into(self) -> Vec<T> {
        self.into_iter().collect::<Vec<T>>()
    }
}

/// Produce an array of T from the index. 
/// 
/// Would love to implement this for:
///     I: SliceIndex<[T]>
/// However, this is proving to be very difficult.
impl<T: Clone> Index<usize> for VecRotate<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let wrapped_index = &self.wrap_index(index);
        &self.array[*wrapped_index]
    }
}

impl<T: Clone> IndexMut<usize> for VecRotate<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let wrapped_index = &self.wrap_index(index);
        &mut self.array[*wrapped_index]
    }
}

impl<'a, T: Debug + Clone> Debug for VecRotate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(
            self.clone().into_iter() )
            .finish()
    }
}