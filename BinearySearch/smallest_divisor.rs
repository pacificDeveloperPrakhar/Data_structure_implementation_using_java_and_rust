pub fn main() {
    let arr = vec![44,22,33,11,1];
    let target: usize = 5;
    
    let result = find_divisor(&arr, target);
    println!("Divisor = {}", result);
}

pub fn find_divisor(arr: &Vec<i32>, target: usize) -> usize {
    let mut s = 1;                  // start from 1 (avoid divide by zero)
    let mut e = *arr.iter().max().unwrap() as usize;
    let mut ans=usize::MAX;
    while s <= e {
        let mid = (e - s) / 2 + s;
        let res= is_satisfied(arr,mid);
        
        if res<= target {        
            println!("{}",mid);    
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
