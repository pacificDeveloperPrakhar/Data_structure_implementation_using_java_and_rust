pub fn find_single(arr:&Vec<i32>, start:usize, end:usize) -> usize {
    if end <= start {
        return start;
    }

    if arr[start] != arr[start + 1] {
        return start;
    }
    else if arr[end] != arr[end - 1] {
        return end;
    }

    find_single(arr, start + 2, end - 2)
}

pub fn main() {
    // Example: every element appears twice except one
    let arr = vec![1, 1, 2, 2, 3, 4, 4, 5, 5];

    let idx = find_single(&arr, 0, arr.len() - 1);

    println!("Array: {:?}", arr);
    println!("Single element is {} at index {}", arr[idx], idx);
}
