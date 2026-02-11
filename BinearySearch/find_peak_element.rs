pub fn find_peak(arr: &Vec<i32>, s: usize, e: usize) -> usize {
    if s > e {
        return usize::MAX;
    }

    let n = arr.len();
    let mid = (e - s) / 2 + s;

    // --- handle boundaries safely ---

    // only one element
    if n == 1 {
        return 0;
    }

    // first element
    if mid == 0 {
        if arr[0] > arr[1] {
            return 0;
        } else {
            return find_peak(arr, mid + 1, e);
        }
    }

    // last element
    if mid == n - 1 {
        if arr[n - 1] > arr[n - 2] {
            return n - 1;
        } else {
            return find_peak(arr, s, mid - 1);
        }
    }

    // --- normal peak condition ---
    if arr[mid] > arr[mid - 1] && arr[mid] > arr[mid + 1] {
        return mid;
    }

    // --- move toward larger neighbour ---
    if arr[mid] < arr[mid - 1] {
        return find_peak(arr, s, mid - 1);
    } else {
        return find_peak(arr, mid + 1, e);
    }
}

pub fn main() {
    let arr = vec![1, 3, 7, 12, 9, 5, 2];

    let peak_index = find_peak(&arr, 0, arr.len() - 1);

    println!("Array: {:?}", arr);

    if peak_index != usize::MAX {
        println!("Peak element is {} at index {}", arr[peak_index], peak_index);
    } else {
        println!("No peak found");
    }
}
