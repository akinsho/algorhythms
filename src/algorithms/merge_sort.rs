fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

/// step 1: start
///
/// step 2: declare array and left, right, mid variable
///
/// step 3: perform merge function.
/// ```
///     if left > right
///         return
///     mid=(left+right)/2
///     merge_sort(array, left, mid)
///     merge_sort(array, mid+1, right)
///     merge(array, left, mid, right)
///```
/// step 4: Stop
pub fn merge_sort<T: Copy + Ord>(data: &mut [T]) {
    let end = data.len();
    let middle = end / 2;
    if end <= 1 {
        return;
    }
    merge_sort(&mut data[0..middle]);
    merge_sort(&mut data[middle..end]);
    let mut new_data: Vec<T> = data.to_vec();
    merge(&data[0..middle], &data[middle..end], &mut new_data[..]);
    data.copy_from_slice(&new_data);
}
