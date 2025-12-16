use std::vec::*;
use std::convert::TryInto;
pub fn main()
{
    let arr:Vec<i64>=vec![1, 2, 3, 4];
    let result=maximum_difference(&arr,2);
    println!("{}",result);
}

pub fn maximum_difference(arr:&Vec<i64>,target:i64)->i64
{
 let mut power:u64=0;
 let mut total_sum=0;
 let mut  count=0;
 for i in 0..arr.len()
 {
  total_sum+=arr[i];
  power=power|(1<<i);
 }
 for i in 0..=power
 {
     let mut iter=i;
     let mut k=0;
     let mut sum=0;
     while iter!=0
     {
         if (iter&1)!=0
         {
             sum+=arr[k];
            }
            iter>>=1;
            k+=1;
            
     }
    println!("{}",sum);
    if (total_sum-2*sum).abs()==target
    {
        count+=1;
    }
 }
 return count/2;
}