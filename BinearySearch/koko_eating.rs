pub fn koko_eating(arr: &Vec<i32>, target: usize) -> usize {
    let mut s: usize = 1;
    let mut e: usize = *arr.iter().max().unwrap() as usize;

    while s <= e {
        let mid = (e - s) / 2 + s;

        if is_satisfied(arr, mid) <= target {
            
            e = mid - 1;
        } else {
            s = mid + 1;
        }
    }
    s
}

pub fn is_satisfied(arr: &Vec<i32>, rate: usize) -> usize {
    let mut total: usize = 0;

    for &pile in arr.iter() {
        // ceil(pile / rate)
        total += (pile as usize + rate - 1) / rate;
    }

    total
}

pub fn main() {
    let arr = vec![805306368,805306368,805306368];
    let h = 1000000000;

    let result = koko_eating(&arr, h);
    println!("Minimum eating speed = {}", result);
}
