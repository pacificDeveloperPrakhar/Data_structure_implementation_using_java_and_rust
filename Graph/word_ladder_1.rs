
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
pub fn difference(word1:&String,word2:&String)->usize
{
    let mut table:Vec<Vec<i32>>=vec![vec![0;word1.len()];word2.len()];
    let word1=word1.as_bytes();
    let word2=word2.as_bytes();
    for i in 0..word2.len()
    {
        if word1[0]==word2[i]
        {
            table[i][0]=1;
        }
        if i>0
        {
            table[i][0]=table[i-1][0];
        }
    }
    for i in 0..word1.len()
    {
        if word1[i]==word2[0]
        {
            table[0][i]=1;
        }
        if i>0
        {
            table[0][i]=table[0][i-1];
        }
    }
    for i in 1..word2.len()
    {
        for j in 1..word1.len()
        {
            
           let mut ans=0;
           ans=maximum(table[i][j-1],table[i-1][j]);
           if word2[i]==word1[j]
           {
               ans=maximum(1+table[i-1][j-1],ans);
           }
           table[i][j]=ans;
        }
    }
    return *[word1.len(),word2.len()].iter().max().unwrap()-(table[word2.len()-1][word1.len()-1] as usize);
}

pub fn maximum(x:i32,y:i32)->i32
{
    if x>y
    {
        return x;
    }
    else
    {
        return y;
    }
}
pub fn word_ladder(words:&Vec<String>,src:usize,tgt:usize)->i32
{
    let mut graph=vec![vec![0;words.len()];words.len()];
    for i in 0..graph.len()
    {
        for j in 0..words.len()
        {
            graph[i][j]=difference(&(words[i]),&(words[j]));
        }
    }

    let mut queue:VecDeque<usize>=VecDeque::new();
    let mut visited=vec![false;words.len()];
    let mut changes=0;
    visited[src]=true;
    queue.push_back(src);
    let mut num_level=1;
    let mut temp=0;
    while queue.is_empty()!=true
    {
        let curr=queue.pop_front().unwrap();
        if curr==tgt
        {
            break;
        }
        for i in 0..num_level
        {
            for j in 0..words.len()
            {
                if j!=curr&&(graph[curr][j]==1)&&(visited[j]!=true)
                {
                    visited[j]=true;
                    queue.push_back(j);
                    temp+=1
                }
            }
        }
        changes+=1;
        num_level=temp;
    }
    return changes;
}
pub fn main() {
    // let words = vec![
    //     "gedk".to_string(),
    //     "geek".to_string(),
    //     "gefk".to_string()
    // ];

    // // "der" -> index 1
    // // "dfs" -> index 4
    // let src = 0;
    // let tgt = 2;

    // let result = word_ladder(&words, src, tgt);
    // println!("{}", result);
    let  w1=String::from("apple");
    println!("{}",*w1);
}
