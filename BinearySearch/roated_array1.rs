pub fn main() {
    // Example rotated sorted array
    // Original sorted: [1, 2, 3, 4, 5, 6, 7]
    // Rotated:         [4, 5, 6, 7, 1, 2, 3]
    let arr = vec![4, 5, 6, 7, 1, 2, 3];

    let target = 2;

    let result = search_rotated(&arr, 0, arr.len() as isize - 1, target);

    if result != -1 {
        println!("Element {} found at index {}", target, result);
    } else {
        println!("Element {} not found", target);
    }
}

pub fn search_rotated(arr:&Vec<i32>, s:isize, e:isize, val:i32) -> isize {
    if e < s {
        return -1;
    }

    let ss = s as usize;
    let ee = e as usize;

    let mid = ((ee - ss) / 2) + ss;
    let mmid = ((e - s) / 2) + s;

    if arr[mid] == val {
        return mmid;
    }

    if arr[mid] > val {
        if (val < arr[mid]) && (arr[ss] <= val) {
            return search_rotated(arr, s, mmid - 1, val);
        } else {
            return search_rotated(arr, mmid + 1, e, val);
        }
    }

    if arr[mid] < val {
        if (val > arr[mid]) && (arr[ee] >= arr[mid]) {
            return search_rotated(arr, mmid + 1, e, val);
        } else {
            return search_rotated(arr, s, mmid - 1, val);
        }
    }

    -1
}
