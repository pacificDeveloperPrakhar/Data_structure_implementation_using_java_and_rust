    use std::vec::*;
    pub fn main()
    {
     let result=coin_change(10,vec![1,2,5]);
     println!("here {}",result);
    }
   
    pub fn coin_change(amount:i64,coins:Vec<i64>)->i64
    {
        for i in 0..coins.len()
        {
            let mut n=(coins.len()-i-1) as isize;
            let mut temp=amount;
            let mut ans=0;
            while (temp>=0)&&(n>=0)
            {
                let mut coins_no=temp/coins[n as usize];
                temp=temp-coins[n as usize]*coins_no;
                ans+=coins_no;
                n-=1;
            }
            if (temp==0)&&(ans>0)
            {
                return ans
            }
        }
        return 0;
    }