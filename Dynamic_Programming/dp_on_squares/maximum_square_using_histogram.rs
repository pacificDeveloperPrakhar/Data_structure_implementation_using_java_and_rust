use std::vec::*;

pub fn main()
{
    let mut sqr: Vec<Vec<i32>> = vec![
    vec![0, 1, 1, 1],
    vec![1, 1, 1, 1],
    vec![0, 1, 1, 1],
];


    let result = total_squares(&mut sqr);
    println!("Total squares: {}", result);
}

pub fn total_squares(sqr:&mut Vec<Vec<i32>>)->i32
{
    let mut count=0;
    for i in 0..sqr.len()
    {
        for j in 0..sqr[0].len()
        {
            if sqr[i][j]==0
            {
                continue;
            }
            count+=count_squares(sqr,i,j);
        }
    }
    return count;
}

pub fn count_squares(sqr:&mut Vec<Vec<i32>>,row:usize,col:usize)->i32
{
 let mut ii:isize=row as isize;
 let mut count=0;
 let mut jj:isize=col as isize;
 let mut is_break=false;
 while(ii>=0)&&(jj>=0)
 {
    let i=ii as usize;
    let j=jj as usize;
    for row_temp in i..=row
    {
        for col_temp in j..=col
        {
            if sqr[row_temp][col_temp]==0
            {
                is_break=true;
                break;
            }
        }
        if is_break
        {
            break;
        }
    }
    if is_break==false
    {
     count+=1;
    }
    else
    {
        is_break=false;
    }
    ii-=1;
    jj-=1;
 }
 return count;
}
