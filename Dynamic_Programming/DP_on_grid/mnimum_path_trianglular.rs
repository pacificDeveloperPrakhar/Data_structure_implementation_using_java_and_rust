use std::vec::*;

pub fn main()
{
    let v: Vec<Vec<i32>> = vec![
    vec![1],
    vec![4, 7],
    vec![4, 10, 50],
    vec![-50, 5, 6, -100],
];


    let result = min_path_sum(v);
    println!("Min path sum: {}", result);
}

pub fn min_path_sum(grid:Vec<Vec<i32>>)->i32
{
    let mut table=vec![vec![i32::MAX;grid[grid.len()-1].len()];grid.len()];
    for ii in 1..=grid.len()
    {
        let mut i=grid.len()-ii;
        for jj in 1..=grid[i].len()
        {
            let j=grid[i].len()-jj;
            if ((i+1)<grid.len())&&((j+1)<grid[i+1].len())
            {
                table[i][j]=minimum(table[i][j],table[i+1][j+1]);
            }
            if (i+1)<grid.len()
            {
                table[i][j]=minimum(table[i][j],table[i+1][j]);
            }
            if table[i][j]==i32::MAX
            {
                table[i][j]=0;
            }
            table[i][j]+=grid[i][j];
        }
    }
    println!("{:?}",table);
    return table[0][0];
}

pub fn minimum(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}
