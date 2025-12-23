use std::vec::*;

pub fn main()
{
    let arr: Vec<i64> = vec![1, 3, 5, 8, 19, 100];
    let target: i64 = 20;

    let result = bineary_search(&arr, 0, arr.len() - 1, target);
    println!("Result: {}", result);
}
pub fn bineary_search(arr: &Vec<i64>, start: usize, end: usize, target: i64) -> i64 {
    if start >= end {
        // Safe nearest comparison
        if start == 0 {
            return arr[start];
        }
        let a = arr[start];
        let b = arr[start - 1];

        if (a - target).abs() < (b - target).abs() {
            return a;
        } else {
            return b;
        }
    }

    let mid: usize = start + (end - start) / 2;
    println!("mid={}", mid);

    if arr[mid] == target {
        return arr[mid];
    }

    if arr[mid] > target {
        return bineary_search(arr, start, mid, target);
    }

    bineary_search(arr, mid + 1, end, target)
}
