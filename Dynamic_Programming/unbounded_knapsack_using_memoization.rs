use std::vec::*;
pub fn main()
{
 let wt=vec![2, 4, 6];
 let vl=vec![5, 11, 13];
 let capacity=8;
 let mut memo=vec![vec![i64::MIN;(capacity+1) as usize];wt.len()];
 let result=unbounded_knapsack(&wt,&vl,capacity,&mut memo);
 println!("{}",result);
}

pub fn unbounded_knapsack(wt:&Vec<i64>,vl:&Vec<i64>,capacity:i64,memo:&mut Vec<Vec<i64>>)->i64
{
 if capacity==0
 {
    return 0;
 }
 let mut ans=i64::MIN;
 for i in 0..wt.len()
 {
     if capacity-wt[i]>=0
     {
     if memo[i as usize][(capacity-wt[i]) as usize]==i64::MIN
     {
        memo[i as usize][(capacity-wt[i]) as usize]=unbounded_knapsack(wt,vl,capacity-wt[i],memo)
     }
     let result=memo[i as usize][(capacity-wt[i]) as usize]+vl[i];
     ans=maximum(ans,result);
    }
 }
 if ans==i64::MIN
 {
    return 0;
 }
 return ans;
}

pub fn maximum(curr:i64,max:i64)->i64
{
    if curr<max
    {
        return max;
    }
    return curr;
}