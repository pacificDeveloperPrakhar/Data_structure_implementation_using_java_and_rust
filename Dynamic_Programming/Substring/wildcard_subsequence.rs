use std::vec::*;

pub fn main()
{
    let reg: Vec<char> = "a".chars().collect();
    let str: Vec<char> = "aa".chars().collect();

    let ii = reg.len() as isize - 1;
    let jj = str.len() as isize - 1;

    let result = wildcard_subsequence(&reg, &str, ii, jj);
    println!("{}", result);
}

pub fn wildcard_subsequence(reg:&Vec<char>,str:&Vec<char>,ii:isize,jj:isize)->bool
{
    if (ii<0)&&(jj<0)
    {
        return true;
    }
    else if ii<0||jj<0
    {
        return false;
    }
    let mut i=ii as usize;
    let mut j=jj as usize;
    
    let mut ans=false;

    if reg[i]=='*'
    {
        ans=ans||wildcard_subsequence(reg,str,ii-1,jj);
        ans=ans||wildcard_subsequence(reg,str,ii-1,jj-1);
        ans=ans||wildcard_subsequence(reg,str,ii,jj-1);
    }
    else if (reg[i]==str[j])||(reg[i]=='?')
    {
        ans=ans||wildcard_subsequence(reg,str,ii-1,jj-1);
    }
    return ans;
}
