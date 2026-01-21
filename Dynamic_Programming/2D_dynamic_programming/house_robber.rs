pub use std::vec::*;

pub fn main()
{
    let mut hs: Vec<i32> = vec![2, 1, 4, 9];
    let result = house_robber(&mut hs, 0);
    println!("Max loot: {}", result);
}

pub fn house_robber(hs:&mut Vec<i32>,idx:usize)->i32
{
    if idx>=hs.len()
    {
        return 0;
    }
    let len=hs.len();
    let mut ans=i32::MIN;
    ans=maximum(house_robber(hs,idx+1),ans); 
    let temp=hs[(len+idx-1)%len];
    hs[(len+idx-1)%len]=0;
    ans=maximum(house_robber(hs,idx+2)+hs[idx],ans);
    hs[(len+idx-1)%len]=temp;
    return ans;
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
