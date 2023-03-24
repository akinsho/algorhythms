pub fn insertion_sort<T: Copy + Ord>(data: &mut [T], steps: &mut Vec<Vec<T>>) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            steps.push(data.to_vec());
            j -= 1;
        }
    }
}
