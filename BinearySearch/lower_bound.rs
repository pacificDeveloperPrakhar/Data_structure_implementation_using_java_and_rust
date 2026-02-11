pub fn main() {
    let arr = vec![1, 3, 5, 7, 9, 11];
    let val = 6;

    let result = lower_bound(&arr, val, 0, arr.len() - 1);

    println!("Lower bound for {} is {}", val, result);
}

pub fn lower_bound(arr: &Vec<i32>, val: i32, start: usize, end: usize) -> i32 {
    if (end<arr.len())&&(end)<= (start+1) {
        return arr[end];
    }

    let mid = start + ((end - start) / 2);

    if arr[mid] < val {
        return lower_bound(arr, val, mid , end);
    } else {
        return lower_bound(arr,val,start,mid);
    }
}
