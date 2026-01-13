use std::vec::*;

pub fn main()
{
    let s = "bababcbadcede";
    let result = pallindrome_split(s);
    println!("Result: {}", result);
}

pub fn pallindrome_split(s:&str)->i32
{
    if s.chars().eq(s.chars().rev())==true
    {
        return 1;
    }
    let mut ans=i32::MAX;
    for i in 0..(s.len()-1)
    {
        ans=minimum(
            pallindrome_split(&s[0..i])
            + pallindrome_split(&s[i+1..(s.len()-1)]),
            ans
        );
    }
    return ans;
}

pub fn minimum(x:i32,y:i32)->i32
{
    if x>y { y } else { x }
}
