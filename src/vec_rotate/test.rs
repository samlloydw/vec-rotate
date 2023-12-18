#[cfg(test)]

use super::*;

#[test]
fn test_initialise() {
    let _ = VecRotate::from(&[1, 2, 3, 4, 5]);
    let _ = VecRotate::new(vec![1, 2, 3, 4, 5]);
    let _ = VecRotate::from(&[1; 5]);
}

#[test]
fn test_empty() {
    let empty: Vec<u32> = Vec::from([]);
    let mut empty_rotate = VecRotate::new(empty);
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
fn test_push() {
    let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3, 4, 5]);
    rotate.push(6);
    let mut pushed: Vec<u32> = rotate.clone().into();
    assert_eq!(pushed, vec![1, 2, 3, 4, 5, 6]);
    rotate.shift_forward(2);
    rotate.push(7);
    pushed = rotate.clone().into();
    assert_eq!(pushed, vec![ 5, 6, 1, 2, 3, 4, 7]);
    rotate.shift_forward(1);
    pushed = rotate.clone().into();
    assert_eq!(pushed, vec![7, 5, 6, 1, 2, 3, 4]);
}

#[test]
fn test_extend() {
    let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3, 4, 5]);
    rotate.extend(&[6, 7, 8]);
    let mut extended: Vec<u32> = rotate.clone().into();
    assert_eq!(extended, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    rotate.shift_forward(3);
    extended = rotate.clone().into();
    assert_eq!(extended, vec![6, 7, 8, 1, 2, 3, 4, 5]);
    rotate.extend(&[9, 10]);
    extended = rotate.clone().into();
    assert_eq!(extended, vec![6, 7, 8, 1, 2, 3, 4, 5, 9, 10]);
}

#[test]
fn test_index() {
    let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3, 4, 5]);
    assert_eq!(rotate[1], 2);
    assert_eq!(rotate.index_via_array(&[0, 1, 2]), vec![1, 2, 3]);
    rotate.shift_forward(2);
    assert_eq!(rotate[1], 5);
    assert_eq!(rotate.index_via_array(&[0, 1, 2]), vec![4, 5, 1]);
}

#[test]
fn test_mutable_index() {
    let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3, 4, 5]);
    rotate.update_via_array(&[1, 2, 3], &[12, 13, 14]);
    let mut mutated: Vec<u32> = rotate.clone().into();
    assert_eq!(mutated, vec![1, 12, 13, 14, 5]);
    rotate.shift_forward(2);
    mutated = rotate.clone().into();
    assert_eq!(mutated, vec![14, 5, 1, 12, 13]);
    rotate.update_via_array(&[0], &[99]);
    mutated = rotate.clone().into();
    assert_eq!(mutated, vec![99, 5, 1, 12, 13]);
}

#[test]
fn test_big_step() {
    let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2]);
    rotate.shift_forward(10);
    rotate.shift_forward(u64::MAX as usize);
    let rotated: Vec<u32> = rotate.clone().into();
    assert_eq!(rotated, vec![2, 1]);
}

#[test]
fn test_small_step_at_end() {
    let mut rotate: VecRotate<u32> = VecRotate::from(&[1, 2, 3]);
    rotate.shift_forward(3);
    assert_eq!(rotate.start_index, 0);
    rotate.shift_backward(3);
    assert_eq!(rotate.start_index, 0);
}

#[test]
fn test_big_array() {
    let mut rotate: VecRotate<u8> = VecRotate::from(&[10; u16::MAX as usize]);
    rotate.shift_forward(3);
    let big_arr: Vec<u8> = rotate.into();
    assert_eq!(vec![10 as u8; u16::MAX as usize], big_arr);
}