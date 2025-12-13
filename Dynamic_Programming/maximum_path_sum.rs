use std::vec::*;

fn main() {
    let mat: Vec<Vec<i64>> = vec![
        vec![1, 2, 10, 4],
        vec![100, 3, 2, 1],
        vec![1, 1, 20, 2],
        vec![1, 2, 2, 1],
    ];

    let rows = mat.len();
    let cols = mat[0].len();

    let max=maximum_path(&mat, rows, cols);
    println!("{}",max);
}

fn maximum_path(mat: &Vec<Vec<i64>>, row: usize, col: usize)->i64 {
    let mut max = i64::MIN;

    let max_row = row as isize;
    let max_col = col as isize;

    let visited = vec![false; row * col];

    for i in 0..row {
        for j in 0..col {
            let mut v = visited.clone();
            v[i * col + j] = true;

            max = maximum(
                max,
                maximum_path_rec(
                    mat,
                    i as isize,
                    j as isize,
                    max_row,
                    max_col,
                    v,
                ),
            );
        }
    }
    return max;
}

fn maximum_path_rec(
    mat: &Vec<Vec<i64>>,
    row: isize,
    col: isize,
    max_row: isize,
    max_col: isize,
    mut visited: Vec<bool>,
) -> i64 {
    let mut max = mat[row as usize][col as usize];

    // up
    if row - 1 >= 0 && !visited[((row - 1) * max_col + col) as usize] {
        let idx = ((row - 1) * max_col + col) as usize;
        visited[idx] = true;
        max = maximum(
            max,
            mat[row as usize][col as usize]
                + maximum_path_rec(mat, row - 1, col, max_row, max_col, visited.clone()),
        );
    }

    // down
    if row + 1 < max_row && !visited[((row + 1) * max_col + col) as usize] {
        let idx = ((row + 1) * max_col + col) as usize;
        visited[idx] = true;
        max = maximum(
            max,
            mat[row as usize][col as usize]
                + maximum_path_rec(mat, row + 1, col, max_row, max_col, visited.clone()),
        );
    }

    // right
    if col + 1 < max_col && !visited[(row * max_col + col + 1) as usize] {
        let idx = (row * max_col + col + 1) as usize;
        visited[idx] = true;
        max = maximum(
            max,
            mat[row as usize][col as usize]
                + maximum_path_rec(mat, row, col + 1, max_row, max_col, visited.clone()),
        );
    }

    // left
    if col - 1 >= 0 && !visited[(row * max_col + col - 1) as usize] {
        let idx = (row * max_col + col - 1) as usize;
        visited[idx] = true;
        max = maximum(
            max,
            mat[row as usize][col as usize]
                + maximum_path_rec(mat, row, col - 1, max_row, max_col, visited.clone()),
        );
    }

    max
}

fn maximum(max: i64, curr: i64) -> i64 {
    if curr > max {
        curr
    } else {
        max
    }
}
