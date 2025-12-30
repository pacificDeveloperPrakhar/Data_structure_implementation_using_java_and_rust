use std::vec::*;

pub fn main() {
    let str1: Vec<char> = "abac".chars().collect();
    let str2: Vec<char> = "cab".chars().collect();

    let result = shortestSupersequence(str1, str2);
    println!("Shortest Supersequence: {}", result);
}

pub fn shortestSupersequence(str1: Vec<char>, str2: Vec<char>) -> String {
    let mut table: Vec<Vec<(i64, String)>> =
        vec![vec![(-1, String::new()); str1.len()]; str2.len()];

    table[0][0].0 = 1;
    table[0][0].1 = str1[0].to_string();

    // first row
    let mut included=false;
    for i in 1..str1.len() {
        if str2[0] == str1[i]{
            if included!=true {
            table[0][i] = table[0][i - 1].clone();
            table[0][i].0+=1;
            table[0][i].1+= &str1[i].to_string();
            included=true;
        }
        continue;
    }
        table[0][i].0 = table[0][i - 1].0 + 1;
        table[0][i].1 = table[0][i - 1].1.clone() + &str1[i].to_string();
    }
    included=false;
    // first column
    for i in 1..str2.len() {
        if str2[i] == str1[0] {
            if included==false
            {
                table[i][0] = table[i - 1][0].clone();
                table[i][0].0+=1;
                table[i][0].1+= &str1[i].to_string();
                included=true;
            }
            continue;
        }
        table[i][0].0 = table[i - 1][0].0 + 1;
        table[i][0].1 = table[i - 1][0].1.clone() + &str2[i].to_string();
    }

    for i in 1..str2.len() {
        for j in 1..str1.len() {
            let (mut skip_value_num, mut skip_value_text) =
                table[i - 1][j - 1].clone();

            if str1[j] == str2[i] {
                skip_value_num += 1;
                skip_value_text += &str1[j].to_string();
            }

            let res1 = table[i][j - 1].clone();
            let res2 = table[i - 1][j].clone();

            if (str1[j]==str2[i])&&(skip_value_num <= res1.0 && skip_value_num <= res2.0) {
                table[i][j] = (skip_value_num, skip_value_text);
            } else if res1.0 < res2.0 {
                table[i][j] = (
                    res1.0 + 1,
                    res1.1 + &str1[j].to_string(),
                );
            } else {
                table[i][j] = (
                    res2.0 + 1,
                    res2.1 + &str2[i].to_string(),
                );
            }
        }
    }
    println!("{:?}",table);
    table[str2.len() - 1][str1.len() - 1].1.clone()
}
