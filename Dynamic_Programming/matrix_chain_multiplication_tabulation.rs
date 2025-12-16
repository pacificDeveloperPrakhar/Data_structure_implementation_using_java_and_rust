use std::vec::*;

pub fn main()
{
     let matrix=vec![1,2,3,4,3];
    let result =matrix_chain_multiplication_refined(&matrix,4);
    println!("{}",result);
}

pub fn matrix_chain_multiplication(matrix:&Vec<i64>,num:usize)->i64
{
    let mut table:Vec<Vec<i64>>=vec![vec![0;num+1];num+1];
    for i in 0..num
    {
        table[i][i+1]=matrix[i]*matrix[i+1]*matrix[i+2];
    }

    for skip in 2..=(num)
    {
     let mut temp=0;
     while (temp+skip+2)<=num
     {
        let first_chain=matrix[temp]*matrix[temp+(skip+1)]*matrix[temp+skip+2];
        let second_chain=matrix[temp+1]*matrix[temp+1+(skip+1)]*matrix[temp+1+skip+2];
        table[temp][temp+skip]=minimum(first_chain+table[temp][temp+(skip-1)],second_chain+table[temp+1][temp+skip]);
        println!("{}=={}",temp,first_chain);
        println!("{}",second_chain);
        temp+=1; 
    }
    }
    println!("{:?}",table);
    return table[0][num];

}

pub fn matrix_chain_multiplication_refined(arr:&Vec<i64>,matrix_num:usize)->i64
{
    let mut table:Vec<Vec<i64>>=vec![vec![0;matrix_num];matrix_num];
    let mut matrix:Vec<Vec<i64>>=vec![vec![0;2];matrix_num];
    for i in 0..matrix_num
    {
     matrix[i][0]=arr[i];
     matrix[i][1]=arr[i+1];
    }
    for i in 0..(matrix_num-1)
    {
        table[i][i+1]=matrix[i][0]*matrix[i+1][0]*matrix[i+1][1];
    }
    for skip in 2..matrix_num
    {
        let mut temp=0;
        while temp+skip<matrix_num
        {
            let first_chain=matrix[temp][0]*matrix[temp+skip][0]*matrix[temp+skip][1]+table[temp][temp+(skip-1)];
            let second_chain=matrix[temp][0]*matrix[temp][1]*matrix[temp+skip][1]+table[temp+1][temp+skip];
            table[temp][temp+skip]=minimum(first_chain,second_chain);
            temp+=1
        }
    }
    return table[0][matrix_num-1];

}

pub fn minimum(max:i64,curr:i64)->i64
{
    if max>curr{
        return curr;
    }
    return max;
}