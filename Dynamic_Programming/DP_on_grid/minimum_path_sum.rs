use std::vec::*;

pub fn main()
{
    let grid: Vec<Vec<i32>> = vec![
        vec![1, 3, 1],
        vec![1, 5, 1],
        vec![4, 2, 1],
    ];

    let result = min_path_sum(grid);
    println!("Min path sum: {}", result);
}

pub fn min_path_sum(grid:Vec<Vec<i32>>)->i32
{
    let mut table=vec![vec![i32::MAX;grid.len()];grid.len()];
    for ii in 1..=grid.len()
    {
        let mut i=grid.len()-ii;
        for jj in 1..=grid[i].len()
        {
            let j=grid[i].len()-jj;
            if (j+1)<grid[i].len()
            {
                table[i][j]=minimum(table[i][j],table[i][j+1]);
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
    return table[0][0];
}

pub fn minimum(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}
