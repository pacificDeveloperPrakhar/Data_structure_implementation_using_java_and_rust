use std::vec::*;
pub fn main()
{
  let cuts: Vec<i32> = vec![5,6,1,4,2];

    let result = cut_rope(9, &cuts);
    println!("Minimum cost: {}", result);
}

pub fn cut_rope(len:usize,cuts:&Vec<i32>)->i32
{
    let mut table=vec![vec![0;len+1];len+1];
    for ii in 0..=len
    {
        let i=len-ii;
        for j in (i+1)..=len
        {
            let mut ans=i32::MAX;
            for k in 0..cuts.len()
            {
                let cut=cuts[k] as usize;
                let mut res=i32::MAX;
                if (i<cut)&&(cut<j)
                {
                    res=table[cut][j]+table[i][cut];
                    ans=minimum(res,ans);
                }
            }
            if ans!=i32::MAX
            {
                table[i][j]=ans+((j-i) as i32);
            }
            else
            {
                table[i][j]=0;
            }
        }
    }
    println!("{:?}",table);
    return table[0][len];
}

pub fn minimum(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

