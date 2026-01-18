use std::vec::*;

pub fn main()
{
    // Example 1
    let cost1 = vec! [1,7,3];
    let perf1 = vec![7,3,5];
    let budget1 = 13;
    let result1 = maximum_performance(&cost1, &perf1, budget1);
    println!("Result 1: {}", result1);

    // Example 2
    let cost2 = vec![2, 3, 4];
    let perf2 = vec![ 15, 25, 35];
    let budget2 = 6;
    let result2 = maximum_performance(&cost2, &perf2, budget2);
    println!("Result 2: {}", result2);

    // Example 3
    let cost3 = vec![1, 3, 5];
    let perf3 = vec![ 5, 10, 50];
    let budget3 = 7;
    let result3 = maximum_performance(&cost3, &perf3,  budget3);
    println!("Result 3: {}", result3);
}

pub fn maximum_performance(cost:&Vec<i32>,perf:&Vec<i32>,budget:usize)->i32
{
    if (budget<=1)||(cost.len()<=0)
    {
        return 0
    }

    let mut table=vec![vec![0;budget];cost.len()];
    for i in 0..budget
    {
        if i>=(cost[0] as usize)
        {
            table[0][i]=perf[0];
        }
    
    }

    for i in 1..cost.len()
    {
        for j in  1..budget
        {
            table[i][j]=maximum(table[i-1][j],table[i][j]);
            if (cost[i] as usize)<=j 
            {
                table[i][j]=maximum(
                    table[i-1][j],
                    table[i-1][j-(cost[i] as usize)]+perf[i]
                );
            }
        }
    }
    println!("table {:?}",table);
    return table[cost.len()-1][(budget-1) as usize];
}

pub fn maximum(x:i32,y:i32)->i32
{
    if x>y { x } else { y }
}
