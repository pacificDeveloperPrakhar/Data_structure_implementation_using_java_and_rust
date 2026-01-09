use std::vec::*;

pub fn main()
{
    let nums: Vec<i32> = vec![5, 1, 4, 2, 3, 6, 8, 7];

    let result = longest_bitonic(&nums, 1, i32::MIN, 0);
    println!("Longest Bitonic Length: {}", result);
}

pub fn longest_bitonic(nums:&Vec<i32>, flag:usize, curr:i32, ptr:usize)->i32
{
    if ptr >= nums.len()
    {
        return 0;
    }

    let mut ans = i32::MIN;
    if flag==1
    {
        ans=maximum(longest_bitonic(nums,0,nums[ptr],ptr),ans);
        if nums[ptr]>=curr
       { ans=maximum(longest_bitonic(nums,flag,nums[ptr],ptr+1)+1,ans);
       }
    }
    else if (flag==0)&&nums[ptr]<=curr
    {
        ans=maximum(longest_bitonic(nums,flag,nums[ptr],ptr+1)+1,ans);
    }
    ans=maximum(longest_bitonic(nums,flag,curr,ptr+1),ans);
    return ans;
    
}

pub fn maximum(curr:i32, max:i32)->i32
{
    if curr > max { curr } else { max }
}
