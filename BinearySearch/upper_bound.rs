pub fn main() {
    let arr = vec![3,5,8,9,15,19];
    let val = 9;

    let result = upper_bound(&arr, val, 0, arr.len() - 1);

    println!("Lower bound for {} is {}", val, result);
}

pub fn upper_bound(arr: &Vec<i32>, val: i32, start: usize, end: usize) -> i32 {
    if (end<arr.len())&&(end)<= (start+1) {
        return arr[end];
    }

    let mid = start + ((end - start) / 2);

    if arr[mid] < val {
        return upper_bound(arr, val, mid+1 , end);
    } else {
        return upper_bound(arr,val,start,mid);
    }
}
