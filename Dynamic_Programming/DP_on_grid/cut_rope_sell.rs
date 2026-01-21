use std::vec::*;

pub fn main() {
    // Example input
    // prices[i] = price of rope of length i+1
    let prices: Vec<i32> = vec![1, 6, 8, 9, 10, 19, 7, 20];
    let n: usize = 8;

    let result = cut_rope(&prices, n);
    println!("Maximum obtainable value: {}", result);
}

pub fn cut_rope(prices: &Vec<i32>, n: usize) -> i32 {
    if n == 1 {
        return prices[0];
    } else if n == 0 {
        return 0;
    }

    let n_piece = n;
    println!("{}",n_piece);
    let mut table = vec![0; n_piece + 1];

    for i in 1..=n_piece {
        let mut ans = i32::MIN;
        ans = maximum(ans, prices[i - 1]);
        for j in 1..i {
            ans = maximum(ans, table[j] + table[i - j]);
        }
        table[i] = ans;
    }
    return table[n];
}

pub fn maximum(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}
