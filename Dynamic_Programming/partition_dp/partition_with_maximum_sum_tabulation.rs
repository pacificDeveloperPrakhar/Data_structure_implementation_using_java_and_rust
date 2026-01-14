use std::vec::*;

pub fn main()
{
    // Example input
    let nums: Vec<i32> = vec![1,4,1,5,7,3,6,1,9,9,3];
    let range: usize = 4;

    let result = partition_with_sum(&nums, range);
    println!("Result: {}", result);
}

pub fn partition_with_sum(nums:&Vec<i32>, range:usize)->i32
{
    if range<=0
    {
        return 0;
    }
    if nums.len() == 0
    {
        return 0;
    }
    else if nums.len()==1
    {
        return nums[0];
    }
    let mut table = vec![vec![0; nums.len()]; nums.len()];
    for i in 0..nums.len()
    {
        table[i][i]=nums[i];
    }

    for ii in 2..=nums.len()
    {
        let i = nums.len() - ii;
        for j in (i+1)..nums.len()
        {
            let subset_range = j - i;
            let mut ans = i32::MIN;

            if subset_range <= (range - 1)
            {
                for k in i..=j
                {
                    ans = maximum(ans, nums[k]);
                }
                ans = ((subset_range + 1) as i32) * ans;
            }

            ans = maximum(ans, nums[i] + table[i + 1][j]);

            for k in (i + 1)..j
            {
                ans = maximum(ans, table[i][k] + table[k + 1][j] );
            }

            ans = maximum(ans, nums[j] + table[i][j - 1]);
            table[i][j] = ans;
        }
    }
    println!("{:?}",table);
    table[0][nums.len() - 1]
}

pub fn maximum(x:i32, y:i32)->i32
{
    if x > y { x } else { y }
}
