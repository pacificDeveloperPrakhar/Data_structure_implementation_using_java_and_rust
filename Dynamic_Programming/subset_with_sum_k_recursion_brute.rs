use std::vec::*;
use std::convert::TryInto;
pub fn main()
{
    let arr=vec![1,2,2,3];
    let target=3;
    let result=subset_with_sum_k(&arr,target,-1,4);
    println!("{}",result);
}

pub fn subset_with_sum_k(arr:&Vec<i64>,target:i64,col:isize,max_col:usize,)->i64
{
    if target==0
    {
        return 1;
    }
    if col==(max_col-1).try_into().unwrap()
        {
            if target==arr[col as usize]
            {
                return 1;
            }
            return 0;
        }
    //include the follwing element into the result
    let mut ans=0;
    if col!=-1
    {
        ans+=subset_with_sum_k(arr,target-arr[col as usize],col+1,max_col);
    }
    let exclude_count=subset_with_sum_k(arr,target,col+1,max_col);
    return ans+exclude_count;
}