pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();

    while right - left > 1 {
        let middle = left + (right - left) / 2;
        if array[middle] == key {
            return Some(middle);
        }
        if array[middle] < key {
            left = middle;
        } else {
            right = middle;
        }
    }

    if !array.is_empty() && array[left] == key {
        return Some(left);
    }
    None
}
