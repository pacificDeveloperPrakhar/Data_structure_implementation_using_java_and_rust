use std::vec::*;
use std::convert::TryInto;

fn main() {
    let mat: Vec<Vec<i64>> = vec![
        vec![1, 2, 10, 4],
        vec![100, 3, 2, 1],
        vec![1, 1, 20, 2],
        vec![1, 2, 2, 1],
    ];

    let rows = mat.len();
    let cols = mat[0].len();
    let mut max = i64::MIN;
    let mut memo: Vec<Vec<i64>> = vec![vec![i64::MIN; cols]; rows];

    for i in 0..cols {
        max = maximum(
            max,
            maximum_path_rec(
                &mat,
                rows,
                cols,
                0,
                i.try_into().unwrap(),
                &mut memo,
            ),
        );
    }

    println!("{:?}", memo);
}

fn maximum_path_rec(
    mat: &Vec<Vec<i64>>,
    max_row: usize,
    max_col: usize,
    row: isize,
    col: isize,
    memo: &mut Vec<Vec<i64>>,
) -> i64 {
    let mut max = i64::MIN;

    if row >= max_row as isize {
        return 0;
    }

    // left-down
    if col - 1 >=0 && ((row+1) as usize) <max_row{
        if memo[(row + 1) as usize][(col - 1) as usize] == i64::MIN {
            memo[(row + 1) as usize][(col - 1) as usize] =
                maximum_path_rec(mat, max_row, max_col, row + 1, col - 1, memo);
        }
        max = maximum(max, memo[(row + 1) as usize][(col - 1) as usize]);
    }

    // down
    if ((row+1) as usize)<max_row
    {

        if memo[(row + 1) as usize][col as usize] == i64::MIN {
            memo[(row + 1) as usize][col as usize] =
            maximum_path_rec(mat, max_row, max_col, row + 1, col, memo);
        }
        max = maximum(max, memo[(row + 1) as usize][col as usize]);
    }

    // right-down
    if col + 1 < max_col as isize &&((row+1) as usize) <max_row{
        if memo[(row + 1) as usize][(col + 1) as usize] == i64::MIN {
            memo[(row + 1) as usize][(col + 1) as usize] =
                maximum_path_rec(mat, max_row, max_col, row + 1, col + 1, memo);
        }
        max = maximum(max, memo[(row + 1) as usize][(col + 1) as usize]);
    }

    mat[row as usize][col as usize] + max
}

fn maximum(max: i64, curr: i64) -> i64 {
    if max > curr {
        max
    } else {
        curr
    }
}
