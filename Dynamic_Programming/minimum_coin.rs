pub fn main()
{   let target=11;
    let coins=vec![1,2,3];
    let mut memo:Vec<i64>=vec![i64::MAX;(target+1) as usize];
    let result=coin_change(&coins,target,&mut memo);
 println!("{}",result);
}

pub fn coin_change(coins:&Vec<i64>,target:i64,memo:&mut Vec<i64>)->i64
{
   
     if target==0
    {
        return 0;
    }
    let mut min=i64::MAX;
    for i in 0..coins.len()
    {
        let mut result=-1;
        if (target-coins[i])>=0
        {

            if memo[(target-coins[i]) as usize]==i64::MAX
            {
                memo[(target-coins[i]) as usize]=coin_change(coins,target-coins[i],memo);
            }
            result=memo[(target-coins[i]) as usize];
        }
        if result==-1
        {
            continue;
        }
        min=minimum(result,min);
    }
    if min==i64::MAX
    {
        return -1;
    }
    return min+1;
}

pub fn minimum(curr:i64,min:i64)->i64
{
    if curr<min
    {
        return curr;
    }
    return min;
}
