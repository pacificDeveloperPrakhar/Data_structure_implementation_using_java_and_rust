use std::vec::*;

pub fn main() {
       let str1: Vec<char> = "axbxax".chars().collect();
    let str2: Vec<char> = "axa".chars().collect();

    let result = shortestSupersequence(str1, str2);
    println!("Shortest Supersequence: {}", result);
}

pub fn shortestSupersequence(str1: Vec<char>, str2: Vec<char>) -> String {
    let n = str2.len();
    let m = str1.len();

    // FIX 1: correct vec initialization syntax
    let mut table = vec![vec![-1; m]; n];

    // first row
    for i in 0..m {
        if str2[0] == str1[i] {
            table[0][i] = 1; // FIX 2: assignment (=), not comparison (==)
            continue;
        }
        if i > 0 {
            table[0][i] = table[0][i - 1];
        }
    }

    // first column
    for i in 0..n {
        if str1[0] == str2[i] {
            table[i][0] = 1;
            continue;
        }
        if i > 0 {
            table[i][0] = table[i - 1][0];
        }
    }

    // DP fill
    for i in 1..n {
        for j in 1..m {
            let mut skip_value_num = table[i - 1][j - 1];

            if str1[j] == str2[i] {
                skip_value_num += 1;
            }

            if str1[j] == str2[i]
                && skip_value_num > table[i - 1][j]
                && skip_value_num > table[i][j - 1]
            {
                table[i][j] = skip_value_num;
            } else if table[i - 1][j] > table[i][j - 1] {
                table[i][j] = table[i - 1][j];
            } else {
                table[i][j] = table[i][j - 1];
            }
        }
    }
    println!("{:?}",table);
    let mut i = (n as isize) - 1;
    let mut j = (m as isize) - 1;
    let mut result = String::new();

    while i >= 0 && j >= 0 {
        let ii = i as usize;
        let jj = j as usize;

        if str1[jj] == str2[ii] {
            result.push(str1[jj]);
            i -= 1;
            j -= 1;
        } else if i > 0 && table[ii - 1][jj] > table[ii][jj - 1] {
            result.push(str2[ii]);
            i -= 1;
        } else {
            result.push(str1[jj]);
            j -= 1;
        }
    }

    while i >= 0 {
        result.push(str2[i as usize]);
        i -= 1;
    }
    while j >= 0 {
        result.push(str1[j as usize]);
        j -= 1;
    }

    result.chars().rev().collect()
}
