pub fn main() {
    // sorted array example
    let arr = vec![3, 4, 13, 13, 13, 20, 40];

    let val1 = 11;
    let val2 = 13;

    let first = first_occurence(&arr, 0, arr.len() - 1, val2);
    let last = last_occurence(&arr, 0, arr.len() - 1, val2);

    let first_print = if first == usize::MAX { -1 } else { first as i32 };
    let last_print = if last == usize::MAX { -1 } else { last as i32 };

    println!("First occurrence of {} = {}", val2, first_print);
    println!("Last occurrence of {} = {}", val2, last_print);

    // test value not present
    let f2 = first_occurence(&arr, 0, arr.len() - 1, val1);
    let l2 = last_occurence(&arr, 0, arr.len() - 1, val1);

    println!(
        "For value {} -> first = {}, last = {}",
        val1,
        if f2 == usize::MAX { -1 } else { f2 as i32 },
        if l2 == usize::MAX { -1 } else { l2 as i32 }
    );
}

pub fn last_occurence(arr: &Vec<i32>, start: usize, end: usize, val: i32) -> usize {
    if start > end {
        return usize::MAX;
    }

    if (start + 1) >= end {
        if arr[end] == val {
            return end;
        }
        if arr[start] == val {
            return start;
        }
        return usize::MAX;
    }

    let mid = (end - start) / 2 + start;

    if arr[mid] == val {
        let right = last_occurence(arr, mid + 1, end, val);
        if right == usize::MAX { mid } else { right }
    } else if arr[mid] < val {
        last_occurence(arr, mid + 1, end, val)
    } else {
        last_occurence(arr, start, mid - 1, val)
    }
}

pub fn first_occurence(arr: &Vec<i32>, start: usize, end: usize, val: i32) -> usize {
    if start > end {
        return usize::MAX;
    }

    if (start + 1) >= end {
        if arr[start] == val {
            return start;
        }
        if arr[end] == val {
            return end;
        }
        return usize::MAX;
    }

    let mid = (end - start) / 2 + start;

    if arr[mid] == val {
        let left = first_occurence(arr, start, mid - 1, val);
        if left == usize::MAX { mid } else { left }
    } else if arr[mid] < val {
        first_occurence(arr, mid + 1, end, val)
    } else {
        first_occurence(arr, start, mid - 1, val)
    }
}
