pub fn closest(arr: &Vec<i32>, val: i32) -> i32 {
    let mut s = 0;
    let mut e = arr.len();

    while s < e {
        let mid = (e - s) / 2 + s;

        let l_c = (arr[mid] - val).abs();
        let r_c = (arr[mid + 1] - val).abs();

        if l_c > r_c {
            s = mid + 1;
        } else {
            e = mid;
        }
    }
    return arr[s];
}

pub fn main() {
    // example sorted array
    let arr = vec![1, 3, 5, 7, 9, 11];

    // value whose closest element we want
    let val = 4;

    let result = closest(&arr, val);

    println!("Closest value to {} is {}", val, result);
}
