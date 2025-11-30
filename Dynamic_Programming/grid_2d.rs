use std::vec::Vec;
pub fn main()->()
{
    

}

pub fn move_fn(memo:& mut Vec<Vec<i8>>,max_col:usize,max_row:usize,col:usize,row:usize)->i8
{
    if col>=max_col||row>=max_row
    {
        return 0;
    }
   if memo[col+1][row]==-1
    {
        memo[col+1][row]=move_fn(& mut memo,max_col,max_row,col+1.row);
    }
   if memo[col][row+1]==-1
    {
        memo[col][row+1]=move_fn(& mut memo,max_col,max_row,col.row+1);
    }

    if memo[col+1][row]>memo[col][row+1]
    {
        return 1+memo[col+1][row];
    }
    else
    {

        return 1+memo[col][row+1];
    }
    
}