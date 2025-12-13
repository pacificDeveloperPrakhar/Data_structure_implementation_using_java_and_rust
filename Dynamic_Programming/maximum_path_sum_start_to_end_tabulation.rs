use std::vec::*;
fn main()
{
 let mat: Vec<Vec<i64>> = vec![
        vec![1, 2, 10, 4],
        vec![100, 3, 2, 1],
        vec![1, 1, 20, 2],
        vec![1, 2, 2, 1],
    ];

    let rows = mat.len();
    let cols = mat[0].len();
    
    let mut tab: Vec<Vec<i64>> = vec![vec![i64::MIN; cols]; rows];
    let max=maximum_path_rec(&mat, &mut tab,rows, cols);
    println!("{:?}",tab);
}

pub fn maximum_path_rec(mat:&Vec<Vec<i64>>,tab:&mut Vec<Vec<i64>>,max_row:usize,max_col:usize)->i64
{

    for i in 0..max_col
    {
     tab[0][i]=mat[0][i];
    }

    let mut max=i64::MIN;
    // now start filling from the top to the very bottom
    for i in 1..max_row
    {
        for j in 0..max_col
        {
            max=i64::MIN;
            if ((j as isize -1) as isize)>=0
            {
                max=maximum(max,tab[i-1][(j-1) as usize]);
            } 
            max=maximum(max,tab[(i-1) as usize][j]);
            if ((j+1) as isize) <max_col as isize
            {
                max=maximum(max,tab[i-1][(j+1) as usize]);
                
            }
            tab[i][j]=max+mat[i][j];
        }
    }
    max=i64::MIN;
    for i in 0..max_col
    {
        max=maximum(max,tab[(max_row-1) as usize][i]);
    }
    return max;
    
}


fn maximum(max: i64, curr: i64) -> i64 {
    if curr > max {
        curr
    } else {
        max
    }
}
