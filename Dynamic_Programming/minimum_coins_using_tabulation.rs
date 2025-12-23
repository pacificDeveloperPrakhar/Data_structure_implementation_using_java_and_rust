use std::vec::*;
pub fn main()
{
    let target=3;
    let coins=vec![2,1];
    let result=coin_change(&coins,target);
    println!("tabulated value={}",result);
}

pub fn coin_change(coins:&Vec<i64>,amount:i64)->i64
{
    let mut table:Vec<i64>=vec![i64::MAX;(amount+1) as usize];
    table[0]=0;
    for i in 0..coins.len()
    {
        table[coins[i] as usize]=1;
    }
    for i in 0..table.len()
    {
        if table[i]!=i64::MAX
        {
            continue;
        }
        for j in 1..=i
        {
            if (table[(i-j) as usize]!=-1)&&table[j]!=-1
            {
             table[i]=minimum(table[(i-j) as usize]+table[j],table[i]);
            }
        }
        if table[i]=i64::MAX;
        table[i]=-1
    }
    println!("{:?}",table);
    return table[amount as usize];

}

pub fn minimum(curr:i64,min:i64)->i64
{
 if curr<min
 {
    return curr;
 }
 return min;
}