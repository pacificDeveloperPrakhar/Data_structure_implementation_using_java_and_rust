use std::vec::*;

pub fn main() {
    let sqr: Vec<Vec<i32>> = vec![
        vec![2, 3, 1, 2],
        vec![3, 4, 2, 2],
        vec![5, 6, 3, 5],
    ];

    let result = ninja_friend(&sqr);
    println!("Max chocolates: {}", result);
}

pub fn ninja_friend(sqr: &Vec<Vec<i32>>) -> i32 {
    let n = sqr.len();
    let m = sqr[0].len();

    let mut table = vec![vec![vec![0; m]; m]; n];

    // Base case: last row
    for i in 0..m {
        for j in 0..m {
            if i == j {
                table[n - 1][i][j] = sqr[n - 1][i];
            } else {
                table[n - 1][i][j] = sqr[n - 1][i] + sqr[n - 1][j];
            }
        }
    }

    let r_temp = vec![-1, 0, 1];

    // Fill DP table bottom-up
    for r in (0..n - 1).rev() {
        for c1 in 0..m {
            for c2 in 0..m {
                let mut ans = i32::MIN;

                for &d1 in &r_temp {
                    for &d2 in &r_temp {
                        let nc1 = c1 as i32 + d1;
                        let nc2 = c2 as i32 + d2;

                        if nc1 < 0 || nc2 < 0 || nc1 >= m as i32 || nc2 >= m as i32 {
                            continue;
                        }

                        ans = maximum(
                            ans,
                            table[r + 1][nc1 as usize][nc2 as usize],
                        );
                    }
                }

                if c1 == c2 {
                    ans += sqr[r][c1];
                } else {
                    ans += sqr[r][c1] + sqr[r][c2];
                }

                table[r][c1][c2] = ans;
            }
        }
    }

    table[0][0][m - 1]
}

pub fn maximum(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}
