use std::vec::*;
pub fn main()
{
    let seq: Vec<i32> = vec![10, 9, 2, 5, 3, 7, 101, 18];

    // ptr = 0, last = very small value so first element can be taken
    let result = increasing_longest_subsequence(&seq, 0, i32::MIN);

    println!("Length of LIS: {}", result);
}

pub fn increasing_longest_subsequence(seq:&Vec<i32>,ptr:usize,last:i32)->i32
{
    if ptr>=seq.len()
    {
        return 0;
    }
    let mut ans=i32::MIN;
    if seq[ptr]>=last
    {
        ans=maximum(increasing_longest_subsequence(seq,ptr+1,seq[ptr])+1,ans);
    }
    ans=maximum(increasing_longest_subsequence(seq,ptr+1,last),ans);
    return ans;
}

pub fn maximum(curr:i32,max:i32)->i32
{
    if curr>max
    {
        return curr;
    }
    return max;
}
