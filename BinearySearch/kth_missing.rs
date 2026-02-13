pub fn kth_missing(arr: &Vec<i32>, target: usize) -> usize {
    let max = *arr.iter().max().unwrap();
    let mut j = 0;
    let mut count = 0;

    for i in 1..=(max as usize) {
        if j < arr.len() && (arr[j] as usize) == i {
            j += 1;
        } else {
            count += 1;
        }

        if count == target {
            return i;
        }
    }

    return count;
}

pub fn main() {
    // example 1
    let arr1 = vec![4,7,9,10];
    let k1 = 1;
    let result1 = kth_missing(&arr1, k1);
    println!("Array: {:?}, k = {} -> kth missing = {}", arr1, k1, result1);

    // example 2
    let arr2 = vec![1, 2, 3, 4];
    let k2 = 2;
    let result2 = kth_missing(&arr2, k2);
    println!("Array: {:?}, k = {} -> kth missing = {}", arr2, k2, result2);

    // example 3
    let arr3 = vec![5, 6, 7];
    let k3 = 3;
    let result3 = kth_missing(&arr3, k3);
    println!("Array: {:?}, k = {} -> kth missing = {}", arr3, k3, result3);
}
