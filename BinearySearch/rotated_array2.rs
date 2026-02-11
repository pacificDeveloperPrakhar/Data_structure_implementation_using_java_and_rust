pub fn main() {
    // Example rotated sorted array (with duplicates)
    let arr = vec![1,0,1,1,1];

    let target = 0;

    let found = search_rotated(&arr, 0, arr.len() as isize - 1, target);

    if found {
        println!("Element {} exists in array", target);
    } else {
        println!("Element {} does NOT exist in array", target);
    }
}

pub fn search_rotated(arr: &Vec<i32>, s: isize, e: isize, val: i32) -> bool {
    if e < s {
        return false;
    }

    let ss = s as usize;
    let ee = e as usize;

    let mmid = (e - s) / 2 + s;
    let mid = (ee - ss) / 2 + ss;

    if arr[mid] == val {
        return true;
    }

    let mut result = false;

    // left sorted
    if arr[mid] > arr[ss] {
        if arr[mid] >= val && val >= arr[ss] {
            result |= search_rotated(arr, s, mmid - 1, val);
        } else {
            result |= search_rotated(arr, mmid + 1, e, val);
        }
    }
    // duplicates on left boundary
    else if arr[mid] == arr[ss] {
        result |= search_rotated(arr, s, mmid - 1, val);
    }


    // right sorted
    if arr[mid] < arr[ee] {
        if arr[mid] <= val && arr[ee] >= val {
            result |= search_rotated(arr, mmid + 1, e, val);
        } else {
            result |= search_rotated(arr, s, mmid - 1, val);
        }
    }
    // duplicates on right boundary
    else if arr[mid] == arr[ee] {
        result |= search_rotated(arr, mmid + 1, e, val);
    }

    result
}
