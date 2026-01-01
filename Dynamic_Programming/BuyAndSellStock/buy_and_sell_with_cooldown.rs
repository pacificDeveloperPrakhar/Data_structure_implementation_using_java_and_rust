use std::vec::*;

pub fn main()
{
    // Example input
    let prices: Vec<i32> = vec![1, 2, 3, 0, 2];

    // day = 0, buy = 0 (can buy), cooldown = 0
    let result = buy_and_sell(&prices, 0, 0, 0);

    println!("Max Profit: {}", result);
}

pub fn buy_and_sell(prices:&Vec<i32>,day:usize,buy:usize,cooldown:usize)->i32
{
    if day>=prices.len()
    {
        return 0;
    }
    let mut ans=i32::MIN;
    if (buy==0)&&(cooldown==0)
    {
        ans=maximum(buy_and_sell(prices,day+1,1,cooldown)-prices[day],ans);
    }
    if buy==1
    {
        ans=maximum(buy_and_sell(prices,day+1,0,1)+prices[day],ans);
    }
    ans=maximum(buy_and_sell(prices,day+1,buy,0),ans);
    return ans;
}

pub fn maximum(curr:i32,max:i32)->i32
{
    if curr>max
    {
        return curr
    }
    return max;
}
