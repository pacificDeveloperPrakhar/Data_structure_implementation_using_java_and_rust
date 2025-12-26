use std::vec::*;
pub fn main()
{
 let result=longest_subsequence_using_tabulation(&("abefg".chars().collect()),&("beg".chars().collect()));
 println!("{:?}",result);
}

pub fn longest_subsequence_using_tabulation(str1:&Vec<char>,str2:&Vec<char>)->String
{
    let mut table=vec![vec![(String::new(),0);str2.len()];str1.len()];
    for i in 0..str1.len()
    {
        if str2[0]==str1[i]
        {
            table[i][0].0=str1[i].to_string();
            table[i][0].1+=1;
        }
    }
    // now for the table creation in the first row
    for i in 1..str2.len()
    {
        if str1[0]==str2[i]
        {
            table[0][i].0=str1[i].to_string();
            table[0][i].1+=1;
            continue;
        }
        table[0][i]=table[0][i-1].clone();

    }
    for i in 1..str1.len()
    {
        for j in 1..str2.len()
        {
            let (mut diagnol_up_text,mut  diagnol_up_num)=table[i-1][j-1].clone();
            if str1[i]==str2[j]
            {
                diagnol_up_num+=1;
                diagnol_up_text=diagnol_up_text+&str1[i].to_string();
            }
            let previous_field=table[i][j-1].clone();
            let skip_field=table[i-1][j].clone();
            if (diagnol_up_num>previous_field.1)&&(diagnol_up_num>skip_field.1)
            {
                table[i][j]=(diagnol_up_text,diagnol_up_num);
            }
            else if skip_field.1>previous_field.1
            {
                table[i][j]=skip_field;
            }
            else
            {
                table[i][j]=previous_field;
            }
        }
    }
    println!("{:?}",table);
    return table[str1.len()-1][str2.len()-1].0.clone();
}