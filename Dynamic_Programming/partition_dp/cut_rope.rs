use std::vec::*;

pub fn main() {
    let cuts: Vec<i32> = vec![1,3,4,5];
    let start: usize = 0;
    let end: usize = 7;
    let mut memo=vec![vec![-1;end+1];end+1];
    let result = cut_rope(start, end, &cuts,&mut memo);
    println!("Minimum cost: {}", result);
}

pub fn cut_rope(start: usize, end: usize, cuts: &Vec<i32>,memo:&mut Vec<Vec<i32>>) -> i32 {
    if end <= start {
        return 0;
    }

    let mut ans = i32::MAX;
    let mut res=i32::MAX;
    for i in 0..cuts.len() {
        let cut = cuts[i] as usize;
        if (start<cut)&&(cut<end)
        {
         if memo[start][cut]==-1
         {
            memo[start][cut]=cut_rope(start,cut,cuts,memo);
         }
         if memo[cut][end]==-1
         {
            memo[cut][end]=cut_rope(cut,end,cuts,memo);
         }
         res=memo[cut][end]+memo[start][cut];
        }
        ans = minimum(
            
                res,
            ans,
        );
    }
    if ans!=i32::MAX
    {
        res=ans+(end - start) as i32;
    }
    else
    {
        res=0;
    }
    return res;
}

pub fn minimum(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}
