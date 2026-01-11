use std::vec::*;

pub fn main() {
    let nums: Vec<i32> = vec![10, 20, 30, 40, 30] ;
    let result = multiplication_chain_root(&nums);
    println!("Result: {}", result);
}

pub fn multiplication_chain_root(nums:&Vec<i32>)->i32
{
    let mut memo=vec![vec![-1;nums.len()];nums.len()];
    if nums.len()<=1
    {
     return 0;
    }
    let mut ans=multiplication_chain(nums,0,nums.len()-2,&mut memo);
    println!("{:?}",memo);
    return ans;
}
pub fn multiplication_chain(nums: &Vec<i32>, start: usize, end: usize,memo:&mut Vec<Vec<i32>>) -> i32 {

    if start >= end {
        return 0;
    }
    let mut ans=i32::MAX;
    for i in start..end {
        if memo[start][i]==-1
        {
            memo[start][i]= multiplication_chain(nums, start, i,memo);
        }
        if memo[i+1][end]==-1
        {
            memo[i+1][end]=multiplication_chain(nums,i+1,end,memo);
        }
        ans = minimum(
            ans,
            nums[start] * nums[i + 1] * nums[end + 1]
                + memo[start][i]
                + memo[i+1][end]
        );
    }

    ans
}

pub fn minimum(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}
