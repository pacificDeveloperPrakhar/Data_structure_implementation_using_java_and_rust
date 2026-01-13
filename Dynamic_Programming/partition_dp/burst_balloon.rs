use std::vec::*;

pub fn main()
{
    let mut nums: Vec<i32> = vec![1];
    let result = burst_baloon(&mut nums,0);
    println!("Result: {}", result);
}

pub fn burst_baloon(nums:&mut Vec<i32>,bursted:usize)->i32
{
    if nums.len()==bursted
    {
        return 0;
    }
    let mut ans=i32::MIN;
    for i in 0..nums.len()
    {
      let temp=nums[i];
      let mut res=temp;
      nums[i]=-1;

      let mut l=i as isize;
      let mut r=i as isize;

      while l>=0 && nums[l as usize]==-1
      {
        l-=1;
      }
      if l>=0
      {
        res=res*nums[l as usize];
      }

      while r<(nums.len() as isize) && nums[r as usize]==-1
      {
        r+=1;
      }
      if r<(nums.len() as isize)
      {
        res=res*nums[r as usize];
      }
      
      ans=maximum(ans,res+burst_baloon(nums,bursted+1));
      println!("{:?}",nums);
      nums[i]=temp;
    }

    ans
}

pub fn maximum(x:i32,y:i32)->i32{
    if x>y { x } else { y }
}
