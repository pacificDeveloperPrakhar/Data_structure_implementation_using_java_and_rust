use std::vec::*;

pub fn main()
{
    let prices = vec![3,2,6,5,0,3];
    let transactions = 2;

    let result = buy_and_sell(&prices, transactions);
    println!("Max Profit: {}", result);
}

pub fn buy_and_sell(prices:&Vec<i32>,transactions:usize)->i32
{
    if transactions==0||prices.len()<=1
    {
        return 0;
    }

    let mut table=vec![vec![vec![0_i32;prices.len()];2];transactions+1];
    for i in 0..prices.len()
    {
        table[1][1][i]=prices[i];
    }
   
    for i in 2..=prices.len()
    {
        let day=prices.len()-i;
        for j in 1..=transactions
        {
            table[j][0][day]=maximum(table[j][0][day+1],table[j][1][day+1]-prices[day]);
            table[j][1][day]=maximum(table[j][1][day+1],table[j-1][0][day+1]+prices[day]);
        }
    }
    println!("{:?}",table);
    return table[transactions][0][0];
}

pub fn maximum(curr:i32,max:i32)->i32
{
    if curr>max
    {
        return curr;
    }
    return max;
}
