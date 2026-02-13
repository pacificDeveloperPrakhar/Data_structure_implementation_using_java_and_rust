pub fn main() {
    let arr = vec![5, 4, 5, 2, 3, 4, 5, 6];
    let target: usize = 5;
    
    let result = find_divisor(&arr, 5);
    println!("Divisor = {}", result);
}

pub fn find_divisor(arr: &Vec<i32>, d: usize,) -> usize {
    let mut s = 1;                  // start from 1 (avoid divide by zero)
    let mut e = *arr.iter().max().unwrap() as usize ;
    let mut ans=usize::MAX;
    while s <= e {
        let mid = (e - s) / 2 + s;
        let res= is_satisfied(arr,mid);
        
        if res<= d {        
            ans=*[ans,mid].iter().min().unwrap();
            e = mid - 1;
        } else {
            s = mid + 1;
        }
    }
    ans
}

pub fn is_satisfied(arr: &Vec<i32>, val: usize) -> usize {
    let mut total = 0;
    for i in 0..arr.len() {
        let mut temp = (arr[i] as usize) / val;
        total += temp;
        if (temp * val) < (arr[i] as usize) {
            total += 1;
        }
    }
 
    total
}
