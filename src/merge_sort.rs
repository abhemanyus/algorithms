pub fn merge_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }
    let (mut arr1, mut arr2) = arr.split_at_mut(arr.len() / 2);
    merge_sort(&mut arr1);
    merge_sort(&mut arr2);
    let merged = merge(arr1, arr2);
    arr.copy_from_slice(&merged);
}

fn merge<T: Copy + PartialOrd>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let len = arr1.len() + arr2.len();
    let mut holder = Vec::with_capacity(len);
    let mut i1 = 0;
    let mut i2 = 0;
    while i1 < arr1.len() && i2 < arr2.len() {
        if arr1[i1] < arr2[i2] {
            holder.push(arr1[i1]);
            i1 += 1;
        } else {
            holder.push(arr2[i2]);
            i2 += 1;
        }
    }
    while i1 < arr1.len() {
        holder.push(arr1[i1]);
        i1 += 1;
    }
    while i2 < arr2.len() {
        holder.push(arr2[i2]);
        i2 += 1;
    }
    holder
}

#[test]
fn test_sort() {
    let mut arr = vec![3, 1, 4, 1, 5, 9];
    let sorted = vec![1, 1, 3, 4, 5, 9];
    merge_sort(&mut arr);
    assert_eq!(arr, sorted);
}
