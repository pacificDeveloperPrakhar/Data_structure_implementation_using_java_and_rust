pub fn find_root(arr: Vec<i32>, val: i32) -> f32 {
    let mut start: usize = 0;
    let mut end: usize = arr.len() - 1;
    let mut root: i32 = 0;

    // integer part using binary search
    while start <= end {
        let mid = (end - start) / 2 + start;
        let sq = arr[mid] * arr[mid];

        if sq == val {
            root = arr[mid];
            break;
        } else if sq > val {
            if mid == 0 {
                break; // prevent usize underflow
            }
            end = mid - 1;
        } else {
            root = arr[mid];
            start = mid + 1;
        }
    }

    // fractional refinement
    let val_f = val as f32;
    let mut result = root as f32;
    let mut increment = 0.1;

    for _ in 0..2 {
        while (result + increment) * (result + increment) <= val_f {
            result += increment;
        }
        increment /= 10.0;
    }

    result
}

fn main() {
    // example array of possible roots (sorted)
    let arr: Vec<i32> = (0..=100).collect();

    let val = 10;

    // let root = find_root(arr, val);
    // println!("Approx sqrt of {} = {:.2}", val, root);
    let mut x_idx:usize=2;
    let mut y_idx=x_idx as isize;
}
