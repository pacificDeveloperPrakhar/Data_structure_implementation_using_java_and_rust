pub fn find_min(arr:&Vec<i32>, start:usize, end:usize) -> usize {
    if end < start {
        return usize::MAX;
    }

    let mut min = 0;
    let mid = (end - start) / 2 + start;

    if arr[mid] > arr[start] {
        if arr[start] < arr[min] {
            min = start;
        }
    } else {
        let idx = find_min(arr, start, mid - 1);
        if idx != usize::MAX && arr[idx] < arr[min] {
            min = idx;
        }
    }
    
    if arr[mid] < arr[end] {
        if arr[mid] < arr[min] {
            min = mid;
        }
    } else {
        let idx = find_min(arr, mid + 1, end);
        if idx != usize::MAX && arr[idx] < arr[min] {
            min = idx;
        }
    }

    min
}

pub fn main() {
    // Rotated sorted array
    // Original: [1, 2, 3, 4, 5, 6, 7]
    // Rotated:  [4, 5, 6, 7, 1, 2, 3]
    let arr = vec![4, 5, 6, 7, 1, 2, 3];

    let min_index = find_min(&arr, 0, arr.len() - 1);

    if min_index != usize::MAX {
        println!("Minimum element is {} at index {}", arr[min_index], min_index);
    } else {
        println!("Could not determine minimum");
    }
}
