use std::vec::*;
pub fn main()
{
    let result = buy_and_sell(&vec![7, 1, 5, 3, 6, 4]);
    println!("\nFinal Result (Max Profit): {}", result);
}

pub fn buy_and_sell(prices:&Vec<i32>)->i32
{
    let mut table=vec![vec![0;2];prices.len()];
    if prices.len()==0
    {
        return 0;
    }

    table[prices.len()-1][1]=prices[prices.len()-1];
    for i in 2..=table.len()
    {
        let mut curr_day=table.len()-i;
        table[curr_day][0]=maximum(table[curr_day+1][0],table[curr_day+1][1]-prices[curr_day]);
        table[curr_day][1]=maximum(table[curr_day+1][1],table[curr_day+1][0]+prices[curr_day]);
    }
    return table[0][0];
}

pub fn maximum(curr:i32,max:i32)->i32
{
    if max>curr
    {
        return max;
    }
    else 
    {
        return curr;
    }
}
