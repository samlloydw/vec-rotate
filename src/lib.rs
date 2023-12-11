use std::{fmt::Debug, ops::{Index, IndexMut}, slice::SliceIndex};

/// Make a vector that can rotate without changing place in memory.
/// This reduces the computational order from O(N) to O(1) for all
/// shifting operations.
#[derive(Clone)]
struct VecRotate<'a, T> {
    slice: &'a [T], // could put in an array instead
    start_index: usize,
    length: usize,
}

impl<'a, T> VecRotate<'a, T> {

    fn new(array: &'a Vec<T>) -> Self {
        let length = array.len();
        VecRotate {
            slice: &array[..],
            start_index: 0,
            length: length,
        }
    }

    /// Check if the vector is empty.
    fn is_empty(&self) -> bool {
        if self.length == 0 {
            true
        } else {
            false
        }
    }
    
    // TODO: Check that no off by one errors are present with using self.length
    /// Shift all the elements forward by 'steps' places.
    /// If the array is empty, don't do anything.
    fn shift_backward(&mut self, steps: usize) -> () {
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

    fn shift_forward(&mut self, steps: usize) -> () {
        if !self.is_empty() {
            let shift_step = steps % self.length;
            if shift_step > self.start_index {
                self.start_index = self.length - (shift_step - self.start_index)
            } else {
                self.start_index -= shift_step;
            }
        }
    }

    fn push(&mut self, _element: T) {
        todo!()
    }
    
    fn extend(&mut self, _extension: &[T]) {
        todo!()
    }

}

impl<'a, T: Clone> IntoIterator for VecRotate<'a, T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self[self.start_index..].iter()
            .chain(self[..self.start_index].iter())
            .cloned().collect::<Vec<T>>().into_iter()
    }
}

impl<'a, T: Clone> From<&'a [T]> for VecRotate<'a, T> {

    fn from(value: &'a [T]) -> Self {
        Self {
            slice: value,
            start_index: 0,
            length: value.len(),
        }
    }
}

impl<'a, T: Clone, const N: usize> From<&'a [T; N]> for VecRotate<'a, T> {
    fn from(value: &'a [T; N]) -> Self {
        Self {
            slice: value.as_slice(),
            start_index: 0,
            length: value.len(),
        }
    }
}

impl<'a, T: Clone> Into<Vec<T>> for VecRotate<'a, T> {
    fn into(self) -> Vec<T> {
        self.into_iter().collect::<Vec<T>>()
    }
}

impl<'a, T, I> Index<I> for VecRotate<'a, T> 
where
    I: SliceIndex<[T], Output = [T]> 
{
    type Output = [T];

    fn index(&self, index: I) -> &Self::Output {
        self.slice.index(index)
    }
}

// impl<'a, T, I> IndexMut<I> for VecRotate<'a, T> 
// where
    // I: SliceIndex<[T], Output = [T]> 
// {
    // fn index_mut(&mut self, index: I) -> &mut Self::Output {
        // self.slice.index_mut(index)
    // }
// }

impl<'a, T: Debug + Clone> Debug for VecRotate<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(
            self.clone().into_iter() )
            // self.slice[self.start_index..].iter().chain(self.slice[..self.start_index].iter()))
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initialise() {
        let _ = VecRotate::from(&[1, 2, 3, 4, 5]);
        let _ = VecRotate::new(&vec![1, 2, 3, 4, 5]);
        let _ = VecRotate::from(&[1; 5]);
    }

    #[test]
    fn test_empty() {
        let empty: Vec<u32> = Vec::from([]);
        let mut empty_rotate = VecRotate::new(&empty);
        assert!(empty_rotate.is_empty());
        empty_rotate.shift_forward(10);
        empty_rotate.shift_backward(10);
    }

    #[test]
    fn test_debug() {
        let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3, 4, 5]);
        println!("Initialisation:\t{:?}", &rotate);
        rotate.shift_forward(2);
        println!("Forward step:\t{:?}", &rotate);
        rotate.shift_backward(2);
        println!("Backward step:\t{:?}", &rotate);
    }

    #[test]
    fn test_clone() {
        let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3, 4, 5]);
        let _: Vec<u32> = rotate.clone().into_iter().collect();
        rotate.shift_forward(3); // test if original obj still exists
    }

    #[test]
    fn test_forward() {
        let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3, 4, 5]);
        rotate.shift_forward(2);
        let mut rotated: Vec<u32> = rotate.clone().into();
        assert_eq!(rotated, vec![4, 5, 1, 2, 3]);
        rotate.shift_forward(5);
        rotated = rotate.clone().into();
        assert_eq!(rotated, vec![4, 5, 1, 2, 3]);
        rotate.shift_forward(21);
        rotated = rotate.clone().into();
        assert_eq!(rotated, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_backward() {
        let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3, 4, 5]);
        rotate.shift_backward(2);
        let mut rotated: Vec<u32> = rotate.clone().into();
        assert_eq!(rotated, vec![3, 4, 5, 1, 2]);
        rotate.shift_backward(5);
        rotated = rotate.clone().into();
        assert_eq!(rotated, vec![3, 4, 5, 1, 2]);
        rotate.shift_backward(14);
        rotated = rotate.clone().into();
        assert_eq!(rotated, vec![2, 3, 4, 5, 1]);
    }

    #[test]
    fn test_no_increments() {
        todo!()
    }


    #[test]
    fn test_big_step() {
        // forwards
        todo!()
        // backwards
    }

    #[test]
    fn test_small_step_at_end() {
        // forwards
        todo!()
        // backwards - at start
    }

    #[test]
    fn test_different_element_types() {
        todo!()
    }

    #[test]
    fn test_big_array() {
        todo!()
    }

}