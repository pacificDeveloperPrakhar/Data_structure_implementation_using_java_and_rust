pub fn grid_ways(m: usize, n: usize) -> u64 {
    let mut tabulation = vec![vec![0u64; m]; n];

    tabulation[0][0] = 1;

    for i in 0..n {
        for j in 0..m {
            if i > 0 {
                tabulation[i][j] += tabulation[i - 1][j];
            }
            if j > 0 {
                tabulation[i][j] += tabulation[i][j - 1];
            }
        }
    }

    tabulation[n - 1][m - 1]
}

pub fn main() {
    println!("{}", grid_ways(3, 2));
}
