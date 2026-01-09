use std::vec::*;
pub fn main()
{

}
pub fn increasing_longest_subsequence(seq:&Vec<i32>)->i32
{
    if seq.len()==1
    {
        return 1
    }
    else if seq.len()<=0
    {
        return 0;
    }
    let mut table=vec![vec![0_i32;seq.len()];seq.len()];
    for i in 0..(seq.len()-1)
    {
        if seq[seq.len()-1]>=seq[i]
        {

            table[i][seq.len()-1]=1;
        }
    }
    for i in 1..seq.len()
    {
        let mut ii=seq.len()-i;
        for j in 2..seq.len()
        {
            let mut jj=seq.len()-j;            
            if seq[jj]<=seq[ii]

        }
    }
}
