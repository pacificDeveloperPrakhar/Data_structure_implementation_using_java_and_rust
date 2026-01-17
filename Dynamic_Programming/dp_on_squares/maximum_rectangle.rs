use std::vec::*;

pub fn main() {
   let sqr = vec![
    vec![1, 0, 1],
    vec![0, 1, 0],
    vec![1, 0, 1],
];



    let rows = sqr.len();
    let cols = sqr[0].len();

    let result = maximum_rectangle_sum(&sqr, 0, cols - 1, 0, rows - 1);
    println!("Result: {}", result);
}

pub fn maximum_rectangle_sum(
    sqr: &Vec<Vec<i32>>,
    col_start: usize,
    col_end: usize,
    row_start: usize,
    row_end: usize,
) -> i32 {
    if (row_start>row_end)||(col_start>col_end)
    {
        return 0;
    }
    let mut count=0;
    let mut result=1;
    for i in row_start..=row_end
    {
        for j in col_start..=col_end
        {
         result&=sqr[i][j];
         count+=sqr[i][j];
        }
    }
    if result!=0
    {
        return count;
    }
    let mut ans=i32::MIN;
    // clipping the first row
    ans=maximum(maximum_rectangle_sum(sqr,col_start,col_end,row_start+1,row_end),ans);
    if row_end>0
    {
        ans=maximum(maximum_rectangle_sum(sqr,col_start,col_end,row_start,row_end-1),ans);
    }
    ans=maximum(maximum_rectangle_sum(sqr,col_start+1,col_end,row_start,row_end),ans);
    if col_end>0
    {
        ans=maximum(maximum_rectangle_sum(sqr,col_start,col_end-1,row_start,row_end),ans);
    }
    return ans;

}

pub fn maximum(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}
