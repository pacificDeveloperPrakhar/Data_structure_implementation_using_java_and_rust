use std::vec::*;
pub fn main()
{
    let reg: Vec<char> = "*?*".chars().collect();
    let str: Vec<char> = "c".chars().collect();
    let result = wildcard_subsequence(&reg, &str);
    println!("{}", result);
}

pub fn wildcard_subsequence(reg:&Vec<char>,sup:&Vec<char>)->bool
{
    let mut table=vec![vec![false;sup.len()];reg.len()];
    let mut count=0;
    if reg[0]!='*'
    {
        count+=1;
    }
    if (reg[0]==sup[0])||(reg[0]=='?')
    {
        table[0][0]=true;
    }
    else if reg[0]=='*'
    {
        for i in 0..sup.len()
        {

            table[0][i]=true;
        }
    }
    for i in 1..reg.len()
    {
     if (reg[i]=='*')&&(table[i-1][0])
     {
         table[i][0]=true;
        }
        else if ((reg[i]==sup[0])||(reg[i]=='?'))&&count==0
        {
            table[i][0]=true;
        count+=1;
    }
    else
    {
        table[i][0]=false;
        count+=1;
    }
}
println!("{}",count);
println!("{:?}",table);
    for i in 1..reg.len()
    {
        for j in 1..sup.len()
        {
            let mut ans=false;
            if reg[i]=='*'
            {
             ans=ans||(table[i-1][j]);
             ans=ans||(table[i-1][j-1]);
             ans=ans||(table[i][j-1]);
            }
            else if (reg[i]==sup[j])||reg[i]=='?'
            {
                ans=ans||(table[i-1][j-1]);
            }
            table[i][j]=ans;
        }
    }
    return table[reg.len()-1][sup.len()-1];
}