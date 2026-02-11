pub fn num_rotated(arr:&Vec<i32>, start:usize, end:usize) -> usize {
    if end < start {
        return 0;
    }

    let mid = (end - start) / 2 + start;

    if arr[mid] > arr[start] {
        // left side sorted — do nothing
    }
    else {
        if mid > 0 && arr[mid - 1] > arr[mid] {
            return mid;
        }
    }

    if arr[mid] < arr[end] {
        // right side sorted — do nothing
    }
    else {
        if mid + 1 < arr.len() && arr[mid + 1] < arr[mid] {
            return arr.len() - (mid + 1);
        }
    }

    0
}

pub fn main() {
    // Example rotated sorted array
    // Original: [1, 2, 3, 4, 5, 6, 7]
    // Rotated:  [5, 6, 7, 1, 2, 3, 4]
    let arr = vec![3,4,5,1,2];

    let rotations = num_rotated(&arr, 0, arr.len() - 1);

    println!("Array: {:?}", arr);
    println!("Number of rotations detected: {}", rotations);
}
