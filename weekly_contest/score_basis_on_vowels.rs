use std::vec::*;

pub fn main()
{
    let s = "cooear";
    let result = cound_score(s);
    println!("Score: {}", result);
}

pub fn cound_score(s:&str)->i32
{
    let chars_in_input:Vec<char>=s.chars().collect();
    let vowels=vec!['a','e','i','o','u'];
    let mut vowel_count=0;
    let mut is_vowel=false;
    let mut consonant_count=0;

    for i in 0..chars_in_input.len()
    {
        let num=chars_in_input[i] as usize;
        let mut is_valid=false;
        if ((num<=90)&&(num>=65))||((num>=97)&&(num<=122))
        {
         is_valid=true;
        }
        if is_valid==false
        {
         continue;
        }

        for j in 0..vowels.len()
        {
            if chars_in_input[i]==vowels[j]
            {
                is_vowel=true;
                break;
            }
        }
        print!("{},",is_vowel);
        if is_vowel
        {
            vowel_count+=1;
            is_vowel=false;
        }
        else
        {
            consonant_count+=1;
        }
    }

    if consonant_count==0
    {
        return 0;
    }
    else
    {
        println!("{}",vowel_count);
        println!("{}",consonant_count);
        return (vowel_count/consonant_count);
    }
}
