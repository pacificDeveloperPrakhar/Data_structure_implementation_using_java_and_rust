use std::vec::*;

pub fn main()
{
    // Standard Burst Balloons input
    // Add 1 at both ends as required by the algorithm
    let coins: Vec<i32> = vec![1, 3, 1, 5, 8, 1];

    let result = burst_balloon(&coins, 1, coins.len() - 2);
    println!("Result: {}", result);
}

pub fn burst_balloon(coins:&Vec<i32>,start:usize,end:usize)->i32
{
    if start>end
    {
        return 0;
    }
    let mut ans=i32::MIN;
    for i in start..=end
    {
     ans=maximum(
        ans,
        coins[start-1]*coins[i]*coins[end+1]
            + burst_balloon(coins,start,i-1)
            + burst_balloon(coins,i+1,end)
     );
    }
    ans
}

pub fn maximum(x:i32,y:i32)->i32
{
    if x > y { x } else { y }
}
