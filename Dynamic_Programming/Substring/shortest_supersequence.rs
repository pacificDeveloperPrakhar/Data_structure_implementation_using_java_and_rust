use std::vec::*;

pub fn main() {
    let s1: Vec<char> = "abefg".chars().collect();
    let s2: Vec<char> = "beg".chars().collect();

    let (len, result) = shortest_supersequence(&s1, &s2, 0, 0);

    println!("Length: {}", len);
    println!("Supersequence: {}", result);
}

pub fn shortest_supersequence(
    str1: &Vec<char>,
    str2: &Vec<char>,
    ptr1: usize,
    ptr2: usize,
) -> (i64, String) {
    // base cases
    if ptr1 >= str1.len() {
        let rem: String = str2[ptr2..].iter().collect();
        return ((str2.len() - ptr2) as i64, rem);
    }

    if ptr2 >= str2.len() {
        let rem: String = str1[ptr1..].iter().collect();
        return ((str1.len() - ptr1) as i64, rem);
    }

    // if characters match
    if str1[ptr1] == str2[ptr2] {
        let (len, mut text) =
            shortest_supersequence(str1, str2, ptr1 + 1, ptr2 + 1);
        text.push(str1[ptr1]);
        return (len + 1, text);
    }

    // skip str1
    let (len1, mut text1) =
        shortest_supersequence(str1, str2, ptr1 + 1, ptr2);
    text1.push(str1[ptr1]);

    // skip str2
    let (len2, mut text2) =
        shortest_supersequence(str1, str2, ptr1, ptr2 + 1);
    text2.push(str2[ptr2]);

    // choose shorter
    if len1 + 1 <= len2 + 1 {
        (len1 + 1, text1)
    } else {
        (len2 + 1, text2)
    }
}
