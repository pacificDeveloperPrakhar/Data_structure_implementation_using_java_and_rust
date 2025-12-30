use std::vec::*;

pub fn main() {
    let str1: Vec<char> = "babgbag".chars().collect();
    let str2: Vec<char> = "bag".chars().collect();

    let m = str1.len() as isize - 1;
    let n = str2.len() as isize - 1;
    let mut memo=vec![vec![-1;str2.len()+1];str1.len()+1];
    let result = distinct_subsequences(&str1, &str2, &mut memo,m, n);
    println!("{:?}",memo);
    println!("Distinct subsequences: {}", result);
}

pub fn distinct_subsequences(
    str1: &Vec<char>,
    str2: &Vec<char>,
    memo:&mut Vec<Vec<i32>>,
    mm: isize,
    nn: isize,
) -> i32 {
    if nn < 0 {
        return 1;
    }
    if mm < 0 {
        return 0;
    }

    let m = mm as usize;
    let n = nn as usize;

    let mut included = 0;
    if str1[m] == str2[n] {
        println!("{}",str2[n]);
        if memo[mm as usize][nn as usize]==-1
        {
            memo[mm as usize][nn as usize]= distinct_subsequences(str1, str2,memo,mm - 1, nn - 1);
        }
        included += memo[mm as usize][nn as usize];
    }
    
    if memo[mm as usize][(nn +1 ) as usize]==-1
    {
        memo[mm as usize][(nn+1) as usize]=distinct_subsequences(str1, str2,memo,mm - 1, nn);
  
    }
    let skip_value_num = memo[mm as usize][(nn+1) as usize];
  

    return included+skip_value_num;
}
