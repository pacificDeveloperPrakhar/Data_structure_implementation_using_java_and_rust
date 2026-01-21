use std::vec::*;

pub fn main()
{
    let mut hs: Vec<i32> = vec![2,3,2];
    let result = house_robber(&mut hs);
    println!("Max loot: {}", result);
}

pub fn house_robber(hs:&mut Vec<i32>)->i32
{
    if hs.len()==0
    {
        return 0;
    }
    else if hs.len()==1
    {
        return hs[0];
    }

    let mut table=vec![0;hs.len()];
    let mut table1=vec![0;hs.len()];
    table[hs.len()-1]=hs[hs.len()-1];
    table1[hs.len()-2]=hs[hs.len()-2];

    for i in 1..(hs.len()-1)
    {
        table[i]=maximum(table[i+1],table[i]);
        if i<table.len()-2
        {
            table[i]=maximum(table[i+2]+hs[i],table[i]);
        }
    }

    for i in 1..(hs.len()-2)
    {
        table1[i]=maximum(table1[i+1],table1[i]);
        if i<table1.len()-2
        {
            table1[i]=maximum(table1[i+2]+hs[i],table1[i]);
        }
    }

    table[0]=maximum(table1[1],table[1]);
    return table[0];
}

pub fn maximum(x:i32,y:i32)->i32
{
    if x>y
    {
        return x;
    }
    else
    {
        return y;
    }
}
