use std::vec::*;
pub fn main()
{
     let prices: Vec<i32> = vec![1, 2, 3, 0, 2];

    // day = 0, buy = 0 (can buy), cooldown = 0
    let result = buy_and_sell(&prices);

    println!("Max Profit: {}", result);
}

pub fn buy_and_sell(prices:&Vec<i32>)->i32{
    let mut table=vec![vec![vec![0_i32;2];2];prices.len()];
    table[prices.len()-1][1][0]=prices[prices.len()-1];

    for i in 2..=prices.len()
    {
        let day=prices.len()-i;
        table[day][1][0]=maximum(table[day+1][0][1]+prices[day],table[day+1][1][0]);
        table[day][0][0]=maximum(table[day+1][1][0]-prices[day],table[day+1][0][0]);
        table[day][0][1]=table[day+1][0][0];
    }
    return table[0][0][0];
}

pub fn maximum(curr:i32,max:i32)->i32
{
    if curr>max
    {
        return curr;
    }
    return max;
}