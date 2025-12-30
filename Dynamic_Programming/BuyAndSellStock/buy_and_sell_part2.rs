use std::vec::*;
pub fn main()
{
    let mut memo=vec![vec![i32::MIN;2];7];
    let result = buy_and_sell(&vec![7, 1, 5, 3, 6, 4], 0, 1,&mut memo);
    println!("\nFinal Result (Max Profit): {}", result);
    println!("table looks like {:?}",memo);
}

pub fn buy_and_sell(prices:&Vec<i32>,day:usize,buy:usize,memo:&mut Vec<Vec<i32>>)->i32
{
    if day>=prices.len()
    {
        return 0;
    }
    let mut ans=i32::MIN;
    if buy==1
    {
        if memo[day+1][0]==i32::MIN
        {
            memo[day+1][0]=buy_and_sell(prices,day+1,0,memo);
        }
        ans=maximum(memo[day+1][0]-prices[day],ans);
    }
    else
    {
        if memo[day+1][1]==i32::MIN
        {
            memo[day+1][1]=buy_and_sell(prices,day+1,1,memo);
        }
        ans=maximum(memo[day+1][1]+prices[day],ans);
    }
    if memo[day+1][buy]==i32::MIN
        {
            memo[day+1][buy]=buy_and_sell(prices,day+1,buy,memo);
        }
    ans=maximum(memo[day+1][buy],ans);
    return ans;
}

pub fn maximum(curr:i32,max:i32)->i32{
    if curr>max
    {
        return curr;
    }
    return max;
}