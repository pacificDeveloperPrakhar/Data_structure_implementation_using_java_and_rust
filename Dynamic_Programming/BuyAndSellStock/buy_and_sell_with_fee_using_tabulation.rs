use std::vec::*;
pub fn main()
{
   let prices: Vec<i32> = vec![1, 3, 2, 8, 4, 9];
    let day: usize = prices.len();
    let fee: i32 = 2;

    let result = buy_and_sell(&prices, fee);
    println!("Max Profit: {}", result);
}

pub fn buy_and_sell(prices:&Vec<i32>,fee:i32)->i32
{
    if prices.len()<=1
    {
        return 0;
    }
    let mut table=vec![vec![0;2];prices.len()];
    table[prices.len()-1][1]=prices[prices.len()-1]-2;

    for i in 2..=prices.len()
    {
        let day=prices.len()-i;
        table[day][0]=maximum(table[day+1][1]-prices[day],table[day+1][0]);
        table[day][1]=maximum(table[day+1][0]+prices[day]-fee,table[day+1][1]);
    }
    println!("{:?}",table);
    return table[0][0];
}
pub fn maximum(curr:i32,max:i32)->i32
{
 if curr>max
 {
    return curr;
 }
 return max;
}