use std::vec::*;

pub fn main() {
    // Example input
    let nums: Vec<i32> = vec![1,1,4,1,5,7,3,6,1,9,9,3,1];
    let k: usize = 3;

    let result = partition_with_sum(&nums, 0, nums.len() - 1, k);
    println!("Result: {}", result);
}

pub fn partition_with_sum(nums:&Vec<i32>, start:usize, end:usize, k:usize) -> i32
{
    if start > end {
        return 0;
    }
    else if start==end
    {
        nums[start];
    }


    let subset_range = end - start;
    let mut ans = i32::MIN;

    if subset_range <= (k - 1) {
        for i in start..=end {
            ans = maximum(ans, nums[i]);
        }
        ans = ((subset_range + 1) as i32) * ans;

    }
    for i in (start)..end {
        ans = maximum(
            partition_with_sum(nums, start, i, k)
            + partition_with_sum(nums, i + 1, end, k),
            ans
        );
    }
    ans
}

pub fn maximum(x:i32, y:i32) -> i32 {
    if x > y { x } else { y }
}
