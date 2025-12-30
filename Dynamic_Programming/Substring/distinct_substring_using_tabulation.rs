use std::vec::*;
pub fn main()
{
    let str1: Vec<char> = "babgbag".chars().collect();
    let str2: Vec<char> = "bag".chars().collect();
    let m = str1.len() as isize - 1;
    let n = str2.len() as isize - 1;
    let result = distinct_subsequences(&str1, &str2);
    println!("{}",result);
}

pub fn distinct_subsequences(sup:&Vec<char>,sub:&Vec<char>)->i64
{
    let mut table=vec![vec![0;sup.len()];sub.len()];
    for i in 0..sup.len()
    {
     if i>=1
     {
        table[0][i]=table[0][i-1];
        
     }
     if sup[i]==sub[0]
     {
        table[0][i]+=1;
     }
    }

    for i in 1..sub.len()
    {
        for j in 1..sup.len()
        {
            let mut result=0;
            if sub[i]==sup[j]
            {
                result+=table[i-1][j-1];
            }
            result+=table[i][j-1];
            table[i][j]=result;
        }
    }
    println!("{:?}",table);
    return table[sub.len()-1][sup.len()-1];
}