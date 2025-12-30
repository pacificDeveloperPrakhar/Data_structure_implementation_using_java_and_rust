use std::vec::*;
pub fn main()
{
 let mut result=buy_and_sell(&vec![7,1,5,3,6,4]);
 println!("{}",result);
}

pub fn buy_and_sell(stocks:&Vec<i32>)->i32
{
 let mut ans=0;
 for i in 0..stocks.len()
 {
    let mut  max_stock_price=0;
    for j in i..stocks.len()
    {
     max_stock_price=maximum(stocks[j],max_stock_price);
    }
    ans=maximum(max_stock_price-stocks[i],ans);
 }
 return ans;
}

pub fn maximum(curr:i32,max:i32)->i32
{
    if curr>max
    {
        return curr;
    }
    return max;
}