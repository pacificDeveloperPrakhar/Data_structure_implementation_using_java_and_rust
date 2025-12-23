use std::vec::*;
pub fn main()
{
    let arr=vec![1,2,3,4];
    let target:i64=5;
    let mut tab=vec![vec![0;target as usize +1];arr.len()];
    let result=subset_with_k(&arr,target,&mut tab);
    println!("{:?}",tab);
}

pub fn subset_with_k(arr:&Vec<i64>,target:i64,tab:&mut Vec<Vec<i64>>)->i64
{
    tab[0][0]=1;
    for i in 1..=(target as usize)
    {
        if i==(arr[0] as usize)
        {
            tab[0][i]=1;
        }  
    }
    println!("{:?}",tab);
    

        for i in 1..tab.len() {
    for j in 0..=target as usize {
        tab[i][j] = tab[i - 1][j];

        if j >= arr[i] as usize {
            tab[i][j] += tab[i - 1][j - arr[i] as usize];
        }
    
}

 }

 return tab[(arr.len()-1) as usize][target as usize];
}