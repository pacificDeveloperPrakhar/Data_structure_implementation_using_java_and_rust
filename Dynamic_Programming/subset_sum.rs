fn main() {
    // The array requested
    let arr = vec![2,3,7,5,6,1];

    // target sum you want to test
    let target = 8;

    // start with empty working vector
    let mut curr_arr: Vec<i64> = Vec::new();

    // this will store all resulting subsets
    let mut ans: Vec<Vec<i64>> = Vec::new();

    // Call your function
    subset_sum(&arr, 0, target, 0, &mut curr_arr, &mut ans);

    // Print the results
    println!("All subsets summing to {}: {:?}", target, ans);
}

pub fn subset_sum(arr:&Vec<i64>,curr_idx:usize,target:i64,sum:i64,curr_arr:&mut Vec<i64>,ans:&mut Vec<Vec<i64>>)
{
    if sum ==target
    {
        ans.push(curr_arr.clone());
        return ();
    }
    if sum>=target||(curr_idx>=arr.len())
    {   
        return ();
    }
    {
        let sum=sum+arr[curr_idx];
        curr_arr.push(arr[curr_idx]);
        subset_sum(arr,curr_idx+1,target,sum, curr_arr, ans);
    }
    curr_arr.pop();
    subset_sum(arr,curr_idx+1,target,sum, curr_arr, ans);



}