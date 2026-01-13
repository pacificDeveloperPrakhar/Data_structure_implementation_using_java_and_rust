use std::vec::*;

pub fn main()
{
    // Standard Burst Balloons input
    let mut coins: Vec<i32> = vec![3, 1, 5, 8];

    let result = burst_balloon(&mut coins);
    println!("Result: {}", result);
}

pub fn burst_balloon(coins:&mut Vec<i32>)->i32
{   if coins.len()==0
    {
        return 0;
    }
    let mut table=vec![vec![0;coins.len()+2];coins.len()+2];
    coins.push(1);
    coins.insert(0,1);
    let start=1;
    let end=coins.len()-2;
    println!("{:?}",coins);
    for i in start..=end
    {
        table[i][i]=coins[i]*coins[i-1]*coins[i+1];
    }

    for ii in start..=end
    {
        let mut i=(end+start)-ii;
        for j in (i+1)..=end
        {
            let mut ans=i32::MIN;
            for k in i..=j
            {
             ans=maximum(ans,table[i][k-1]+table[k+1][j]+coins[k]*coins[i-1]*coins[j+1]);
            }
            table[i][j]=ans;
        }
    }
    
    println!("{:?}",table);
    table[1][table.len()-2]
}

pub fn maximum(x:i32,y:i32)->i32
{
    if x>y { x } else { y }
}
