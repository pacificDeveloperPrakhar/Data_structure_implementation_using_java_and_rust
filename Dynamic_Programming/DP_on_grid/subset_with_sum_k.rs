use std::vec::*;

pub fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 3];
    let target: usize = 6;

    let result = subset_with_count(nums, target);
    println!("Subset count: {}", result);
}

pub fn subset_with_count(nums: Vec<i32>, target: usize) -> i32 {
    if target == 0 {
        return 1;
    }
    if nums.len() == 0 {
        return 0;
    }

    let mut table = vec![vec![0; target + 1]; nums.len()];

    for i in 0..nums.len() {
        table[i][0] = 1;
    }

    for i in 0..=target {
        if i == (nums[0] as usize) {
            table[0][i] = 1;
        }
    }

    for i in 1..nums.len() {
        for j in 1..=target {
            table[i][j] += table[i - 1][j];
            if nums[i] as usize <= j {
                table[i][j] += table[i - 1][j - (nums[i] as usize)];
            }
        }
    }

    table[nums.len() - 1][target]
}

pub fn maximum(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}
