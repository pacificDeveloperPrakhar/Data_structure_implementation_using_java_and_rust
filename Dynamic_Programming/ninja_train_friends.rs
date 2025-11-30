pub fn main()
{

}

pub fn ninja_train_friends(data:&Vec<Vec<u64>>,memo:&mut Vec<Vec<Vec<Option<u64>>>>,days:usize,cols:usize,ninja1_step:usize,ninja2_step:usize,curr_day:usize)->u64
{
    if curr_day==(days-1)
    {
     if memo[curr_day][ninja1_step][ninja2_step]==-1
     {
        if ninja1_step !=ninja2_step
        {
            memo[curr_day][ninja1_step][ninja2_step]=Some(data[curr_day][ninja1_step]+data[curr_day][ninja2_step]);
        }
        else if ninja1_step==ninja2_step
        {
            memo[curr_day][ninja1_step][ninja2_step]=Some(data[curr_day][ninja1_step]);
        }
     }
     return memo[curr_day][ninja1_step][ninja2_step];
    } 

    let step_mod=[-1,0,1];
    let mut max:u64=u64::MIN;
    for i in 0..3
    {
        for j in 0..3
        {         
          let ninja1_step=if ninja1_step+step_mod[i]>=0 {ninja1_step+step_mod[i]} else {(ninja1_step+step_mod[i]).clamp(0,cols)};
          let ninja2_step=if ninja2_step+step_mod[j]>=0 {ninja2_step+step_mod[j]} else {(ninja2_step+step_mod[j]).clamp(0,cols)};

          if memo[curr_day][ninja1_step][ninja2_step]!=-1
          {
            

            memo[curr_day+1][ninja1_step][ninja2_step]=ninja_train_friends(data,memo,ninja1_step,ninja2_step);

          }
          if max<memo[curr_day][ninja1_step][ninja2_step]
          {
            max=memo[curr_day][ninja1_step][ninja2_step];
          }
        }
    }
    if ninja1_step!=ninja2_step
    {

        return max+data[curr_day][ninja1_step]+data[curr_day][ninja2_step];
    }
    return  max+data[curr_day][ninja1_step];
}