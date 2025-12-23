use std::vec::*;
pub fn main()
{
     let arr=vec![1,2,5];
     let target=500;
     let mut memo:Vec<Vec<i64>>=vec![vec![-1;(target+1) as usize];arr.len()];    
     let result=total_coins_exchange(target,&arr,&mut memo,arr.len() as isize-1);
     println!("here {}",result);
}

pub fn total_coins_exchange(mut amount:i64,coins:&Vec<i64>,memo:&mut Vec<Vec<i64>>,n:isize)->i64
{
    if amount==0
    {
        return 1;
    }
    
    let remaining_amount=amount-coins[n as usize];
    let mut include=0;
    if remaining_amount>=0
    {
        if memo[n as usize][remaining_amount as usize]==-1
        {
            memo[n as usize][remaining_amount as usize]=total_coins_exchange(remaining_amount,coins,memo,n);
        }
        include=memo[n as usize][remaining_amount as usize];
    }

    let mut skip=0;
    if n>0
    {
        if memo[(n-1) as usize][amount as usize]==-1
        {
            memo[(n-1) as usize][amount as usize]=total_coins_exchange(amount,coins,memo,n-1);
        }
        skip=memo[(n-1) as usize][amount as usize];
    }

    return skip+include;

}


