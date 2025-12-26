use std::vec::*;
pub fn main()
{
 let str1="xcnnnilop";
 let str2="pnip";
 let mut memo=vec![vec![i64::MIN;str2.len()];str1.len()];
 let result =longest_substring(&str1.chars().collect(),&str2.chars().collect(),0,0,&mut memo);
 println!("{:?}",memo);
}
pub fn longest_substring(str1:&Vec<char>,str2:&Vec<char>,ptr1:usize,ptr2:usize,memo:&mut Vec<Vec<i64>>)->i64
{
    if (ptr1>=str1.len())||(ptr2>=str2.len())
    {
     return 0;
    }
    if str1[ptr1]==str2[ptr2]
    {
        return 1+longest_substring(str1,str2,ptr1+1,ptr2+1,memo);
    }

    let mut ans=0;
    if ptr1<(str1.len()-1)
    {
    if memo[ptr1+1][ptr2]==i64::MIN
    {
        memo[ptr1][ptr2]=longest_substring(str1,str2,ptr1+1,ptr2,memo);
    }
    ans=maximum(ans,memo[ptr1+1][ptr2]);
    }

    if ptr2<(str2.len()-1)
    {
        if memo[ptr1][ptr2+1]==i64::MIN
        {
            memo[ptr1][ptr2+1]=longest_substring(str1,str2,ptr1,ptr2+1,memo);
        }
        ans=maximum(ans,memo[ptr1][ptr2+1]);
    }
    return ans;
}

fn maximum(max: i64, curr: i64) -> i64 {
    if curr > max {
        curr
    } else {
        max
    }
}
