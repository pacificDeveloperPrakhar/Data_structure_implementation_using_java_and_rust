use std::vec::*;

pub fn main()
{
    let s = "bdfe";
    let result = pallindrome_split(&s);
    println!("Result: {}", is_pallindrome(&s));
}

pub fn pallindrome_split(s:&str)->i32
{
    if s.len()==0
    {
        return 0;
    }

    let mut table=vec![vec![0;s.len()];s.len()];

    for ii in 1..=table.len()
    {
        let i=table.len()-ii;
        for j in i..table.len()
        {
            let mut ans=i32::MAX;
            if s[i..=j].chars().eq(s[i..=j].chars().rev())==true
            {
                ans=0;
            }
            else
            {
                for k in i..j
                {
                    ans=minimum(ans,table[i][k]+table[k+1][j]+1)
                }
            }
            table[i][j]=ans;
        }
    } 
    table[0][table.len()-1]
}

pub fn minimum(x:i32,y:i32)->i32
{
    if x>y { y } else { x }
}

