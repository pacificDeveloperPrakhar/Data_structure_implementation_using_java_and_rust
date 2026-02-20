pub fn main() {

    let mut arr4 = vec![7,2,5,10,8];
    println!("Answer = {}", partition(&mut arr4, 2));
}

pub fn partition(arr: &mut Vec<i32>, painters: usize) -> i32 {
    // total sum of boards
    let total: i32 = arr.iter().sum();

    // search range
    let mut s = *arr.iter().max().unwrap(); // minimum possible max work
    let mut e = total;                      // maximum possible max work

    // binary search
    while s < e {
        let mid = (e - s) / 2 + s;

        if is_allocated(arr, mid, painters) {
            e = mid ;
        } else {
            s = mid+1;
        }
    }
 return e;    
}

pub fn is_allocated(arr: &Vec<i32>, val: i32, painters: usize) -> bool {
    let mut used_painters = 1;
    let mut current_sum = 0;
    let mut  max_sum=i32::MIN;
    for i in 0..arr.len()
    {
        if arr[i]>val
        {
            return false;
        }
        if arr[i]+current_sum>val
        {
            used_painters+=1;
            max_sum=*[current_sum,max_sum].iter().max().unwrap();
            current_sum=arr[i];
        }
        else
        {
            current_sum+=arr[i];
        }
    }
    println!("{} {}",max_sum,used_painters);
    if painters>=used_painters
    {
        return true;
    }
    else
    {
        return false;
    }
}