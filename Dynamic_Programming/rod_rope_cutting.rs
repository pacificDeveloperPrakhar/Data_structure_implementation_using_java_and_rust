use std::vec::*;

pub fn main() {
    let rope: i64 = 4;
    let prices = vec![1, 5, 8, 9];
    let mut memo = vec![vec![i64::MIN; (rope + 1) as usize]; prices.len()];

    let result = rope_cut(&prices, &mut memo, rope);
    println!("{}", result);
}

pub fn rope_cut(prices: &Vec<i64>, memo: &mut Vec<Vec<i64>>, rope: i64) -> i64 {
    if rope == 0 {
        return 0;
    }

    let mut ans = 0;

    for i in 1..=rope {
        if memo[(i-1) as usize][(rope-i) as usize]==i64::MIN
        {
            memo[(i-1) as usize][(rope-i) as usize]=rope_cut(prices, memo, rope - i) + prices[(i - 1) as usize];
        }
        let result = memo[(i-1) as usize][(rope-i) as usize];
        ans = maximum(ans, result);
    }

    ans
}

pub fn maximum(curr: i64, max: i64) -> i64 {
    if max < curr {
        return curr;
    }
    max
}
