use std::vec::*;

pub fn main() {
    let str1: Vec<char> = "rabbbit".chars().collect();
    let str2: Vec<char> = "rabbit".chars().collect();

    let result = distinct_supersequences(&str1, &str2);
    println!("Shortest Supersequence: {}", result);
}

pub fn distinct_supersequences(str1: &Vec<char>, str2: &Vec<char>) -> i64 {
    let mut table: Vec<Vec<i64>> = vec![vec![-1; str1.len()]; str2.len()];

    for i in 0..str1.len() {
        if str1[i] == str2[0] {
            table[0][i] = 1;
            continue;
        }
        if i > 0 {
            table[0][i] = table[0][i - 1];
        }
    }

    for i in 0..str2.len() {
        if str1[0] == str2[i] {
            table[i][0] = 1;
            continue;
        }
        if i > 0 {
            table[i][0] = table[i - 1][0];
        }
    }

    for i in 1..str2.len() {
        for j in 1..str1.len() {
            let mut skip_value_num = table[i - 1][j - 1];
            if str1[j] == str2[i] {
                skip_value_num += 1;
            }

            if (str1[j] == str2[i])
                && (skip_value_num > table[i - 1][j])
                && (skip_value_num > table[i][j - 1])
            {
                table[i][j] = skip_value_num;
            } else if table[i - 1][j] > table[i][j - 1] {
                table[i][j] = table[i - 1][j];
            } else {
                table[i][j] = table[i][j - 1];
            }
        }
    }

    println!("{:?}", table);
    supersequence_rec(&mut table, str1, str2, str2.len() - 1, str1.len() - 1)
}

pub fn supersequence_rec(
    table: &mut Vec<Vec<i64>>,
    str1: &Vec<char>,
    str2: &Vec<char>,
    i: usize,
    j: usize,
) -> i64 {
    if i == 0 || j == 0 {
        return 0;
    }

    let mut count = 0;
    let mut skip_value_num = table[i - 1][j - 1];

    if str1[j] == str2[i] {
        skip_value_num += 1;
    }

    if (str1[j] == str2[i]) && (skip_value_num == table[i][j]) {
        if table[i][j] == (j ) as i64 {
            count += 1 + supersequence_rec(table, str1, str2, i - 1, j - 1);
        }
    }

    if table[i][j] == table[i][j - 1] {
        if table[i][j] == (j ) as i64 {
            count += supersequence_rec(table, str1, str2, i, j - 1);
        }
    }

    if table[i][j] == table[i - 1][j] {
        if table[i][j] == (j ) as i64 {
            count += supersequence_rec(table, str1, str2, i - 1, j);
        }
    }

    count
}
