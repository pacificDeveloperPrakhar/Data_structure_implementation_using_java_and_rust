use std::vec::*;

pub fn main()
{
    // Example binary matrix
    let sqr: Vec<Vec<i32>> =  vec![
    vec![0, 1, 1, 1],
    vec![1, 1, 1, 1],
    vec![0, 1, 1, 1],
];

    let result = maximum_rectangle_gph(&sqr);
    println!("Maximum Rectangle Area: {}", result);
}

pub fn maximum_rectangle_gph(sqr:&Vec<Vec<i32>>)->i32
{
    if sqr.len()<=0
    {
        return 0;
    }
    let mut gph=vec![0;sqr[0].len()];
    let mut ans=i32::MIN;
    for i in 0..sqr.len()
    {
      for j in 0..sqr[i].len()
      {
        if sqr[i][j]==0
        {
            gph[j]=0;
        }
        else
        {
            gph[j]+=sqr[i][j];
        }
      }
      ans=maximum(ans,maximum_rectangle_histogram(&gph,0,sqr[i].len()-1));
    }
    return ans;
}

pub fn maximum_rectangle_histogram(gph:&Vec<i32>,l:usize,r:usize)->i32
{
    if l>r
    {
        return 0;
    }
    let mut height=i32::MAX;
    let mut ans=i32::MIN;
    let mut height_idx=l;
    for i in l..=r
    {
        height=minimum(height,gph[i]);
        if height==gph[i]
        {
            height_idx=i
        }
    }
    ans=maximum(maximum_rectangle_histogram(gph,height_idx+1,r),ans);
    if height_idx>0
    {
        ans=maximum(maximum_rectangle_histogram(gph,l,height_idx-1),ans);
    }
    ans=maximum(ans,height*((r-l+1) as i32));
    return ans;
}

pub fn maximum(x:i32,y:i32)->i32
{
    if x>y { x } else { y }
}

pub fn minimum(x:i32,y:i32)->i32
{
    if x>y { y } else { x }
}
