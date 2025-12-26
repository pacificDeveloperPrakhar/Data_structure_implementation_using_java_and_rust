use std::vec::*;

pub fn main() {
    // dummy input
    let str1: Vec<char> = "xcnnnilop".chars().collect();
    let str2: Vec<char> = "pnip".chars().collect();

    let result = longest_substring(str1, str2);
    println!("Result: {}", result);
}

pub fn longest_substring(str1: Vec<char>, str2: Vec<char>) -> i64 {
    let mut max_idx = 0;
    let mut max_val = 0;
    let mut table: Vec<Vec<i64>> = vec![vec![-1; str1.len()]; str2.len()];

    for i in 0..str1.len() {
        if str1[i] == str2[0] {
            table[0][i] = 1;
            max_idx = i;
            break;
        }
    }

    for i in 1..str2.len() {
        for j in max_idx..str1.len() {
            max_val = maximum(table[i - 1][j], max_val);
            if max_val == table[i - 1][j] {
                max_idx = j;
            }
        }

        for j in (max_idx + 1)..str1.len() {
            if str1[j] == str2[i] {
                table[i][j] = 1
            }
        }
    }

    let mut ans = 0;
    for i in 0..str1.len() {
        ans = maximum(table[str2.len() - 1][i], ans);
    }

    println!("{:?}",table);
    ans
}

pub fn maximum(curr: i64, max: i64) -> i64 {
    if curr > max {
        curr
    } else {
        max
    }
}
