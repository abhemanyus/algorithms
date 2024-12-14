use std::cmp::Ordering;

pub fn binary_search<T: PartialOrd>(arr: &[T], val: &T) -> Option<usize> {
    let mut i = 0;
    let mut arr = arr;
    loop {
        let mid = arr.len() / 2;
        match arr.get(mid)?.partial_cmp(val)? {
            Ordering::Less => {
                i += mid + 1;
                arr = &arr[mid + 1..];
            }
            Ordering::Equal => return Some(i + (mid)),
            Ordering::Greater => {
                arr = &arr[..mid];
            }
        }
    }
}

#[test]
fn test_search() {
    let arr = vec![1, 1, 2, 3, 5, 8, 13];
    assert_eq!(binary_search(&arr, &3), Some(3));
    assert_eq!(binary_search(&arr, &5), Some(4));
    assert_eq!(binary_search(&arr, &2), Some(2));
}
