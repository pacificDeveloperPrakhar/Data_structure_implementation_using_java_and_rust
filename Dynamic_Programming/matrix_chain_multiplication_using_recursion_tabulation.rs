use std::vec::*;
use std::collections::*;
pub fn main()
{
    let matrix=vec![1,2,3,4,3];
    let mut memo:Vec<Vec<i64>>=vec![vec![i64::MAX;4];4];
    let result =matrix_chain_multiplication(&matrix,&mut memo,0,3);
    println!("{:?}",memo);
    println!("{}",result);

}

pub fn matrix_chain_multiplication(matrix:&Vec<i64>,memo:&mut Vec<Vec<i64>>,start:usize,end:usize)->i64
{
    if (((start as isize)-end as isize)).abs()==1
    {
        return matrix[start]*matrix[end]*matrix[end+1];
    }
    if memo[start+1][end]==i64::MAX
    {
     memo[start+1][end]=matrix_chain_multiplication(&matrix,memo,start+1,end);
    }
    if memo[start][end-1]==i64::MAX
    {
        memo[start][end-1]=matrix_chain_multiplication(&matrix,memo,start,end-1);
    }

    let mut first_chain=matrix[start]*matrix[start+1]*matrix[end+1]+memo[start+1][end];
    let mut last_chain=matrix[start]*matrix[end]*matrix[end+1]+memo[start][end-1];
    return minimum(first_chain,last_chain);
    
}

pub fn minimum(max:i64,curr:i64)->i64
{
    if max>curr{
        return curr;
    }
    return curr;
}