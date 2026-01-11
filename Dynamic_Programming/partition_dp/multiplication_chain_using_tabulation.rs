use std::vec::*;

pub fn main() {
    let nums: Vec<i32> = vec![10, 20, 30, 40, 30];
    let result = multiplication_chain(&nums);
    println!("Result: {}", result);
}

pub fn multiplication_chain(nums: &Vec<i32>) -> i32 {
    let n = nums.len();
    let mut table = vec![vec![0; n]; n];

    for ii in 1..=n
    {
        let mut i=n-ii;
        for j in (i+1)..(n-1)
        {
            let mut ans=i32::MAX;
            for k in i..j
            {
                ans=minimum(table[i][k]+nums[i]*nums[k+1]*nums[j+1]+table[k+1][j],ans);
            }
            table[i][j]=ans;
        }
    }

    println!("{:?}", table);
    table[0][n - 2]
}

pub fn minimum(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}
