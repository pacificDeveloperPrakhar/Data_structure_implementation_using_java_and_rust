use std::collections::{VecDeque, HashMap};
pub fn word_ladder(word_lis:&Vec<String>,begin_word:String,end_word:String)->Vec<Vec<String>>
{
    let mut word_lis=word_lis.clone();
    let mut graph:HashMap<String,Vec<usize>>=HashMap::new();
    let mut lookup:HashMap<usize,Vec<String>>=HashMap::new();
    let mut tgt=usize::MAX;
    let mut src=usize::MAX;
    for i in 0..word_lis.len()
    {
     if (src==usize::MAX)&&word_lis[i]==begin_word
        {
            src=i;
        }
         if tgt==(usize::MAX)&&word_lis[i]==end_word
        {
            tgt=i;
        }
    }
    if tgt==usize::MAX
    {
        return Vec::new();
    }
    if src==usize::MAX
    {
     word_lis.push(begin_word.clone());
     src=word_lis.len()-1;
    }
    // create the graph
    for i in 0..word_lis.len()
    {
        
        for j in 0..word_lis[i].len()
        {
            let mut bytes = word_lis[i].as_bytes().to_vec();
            bytes[j] = b'*';

            let word_m = String::from_utf8(bytes).unwrap(); 

            lookup.entry(i)
            .or_insert(Vec::new())
            .push(word_m.clone());

            
            graph.entry(word_m.clone())
            .or_insert(Vec::new())
            .push(i);
        }
}
let mut visited = vec![false; word_lis.len()];

    let mut ans: Vec<Vec<String>> = Vec::new();
    let mut  curr_ans: Vec<String> = Vec::new();
    curr_ans.push(word_lis[src].clone());
    // Call recursive path finder
    let shortes_path=word_ladder_rec(
        &word_lis,
        &graph,
        &lookup,
        visited,
        curr_ans,
        &mut ans,
        src,
        tgt,
        usize::MAX
    );
  ans.retain(|x|x.len()==shortes_path);
  return ans;
}

pub fn minimum(x:usize,y:usize)->usize
{
    if x>y
    {
        return y;
    }
    else
    {
        return x;
    }
}
pub fn word_ladder_rec(word_lis:&Vec<String>,graph:&HashMap<String,Vec<usize>>,lookup:&HashMap<usize,Vec<String>>,mut visited: Vec<bool>,mut curr_ans:Vec<String>,ans:&mut Vec<Vec<String>>,src:usize,tgt:usize,mut min_path:usize)->usize
{
 visited[src]=true;
 if curr_ans.len()>min_path
 {
    return min_path;
 }
 if word_lis[src]==word_lis[tgt]
 {
    min_path=curr_ans.len();
    ans.push(curr_ans);
    return min_path;
 }
 let lookup_vec=lookup.get(&src).unwrap();
 for i in 0..lookup_vec.len()
 {
    let curr_neigh=graph.get(&lookup_vec[i]).unwrap();
    for j in 0..curr_neigh.len()
    {
        if visited[curr_neigh[j]]!=true
        {
            let mut new_clone=curr_ans.clone();
            new_clone.push(word_lis[curr_neigh[j]].clone());
            min_path=word_ladder_rec(word_lis,graph,lookup,visited.clone(),new_clone,ans,curr_neigh[j],tgt,min_path);
        }
    }
 }
 return min_path;
}

pub fn main() {
    // ===== Test case 1: your original example =====
    let word_list1 = vec![
        "des".to_string(),
        "der".to_string(),
        "dfr".to_string(),
        "dgt".to_string(),
        "dfs".to_string(),
    ];

    let start1 = "der".to_string();
    let target1 = "dfs".to_string();

    let paths1 = word_ladder(&word_list1, start1, target1);

    println!("All transformation paths (Test 1):");
    for path in paths1 {
        println!("{:?}", path);
    }

    println!("----------------------------------");

    // ===== Test case 2: hit -> hot -> lot/log -> cog =====
    let word_list2 = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
    ];

    let start2 = "hit".to_string();
    let target2 = "cog".to_string();

    let paths2 = word_ladder(&word_list2, start2, target2);

    println!("All transformation paths (Test 2):");
    for path in paths2 {
        println!("{:?}", path);
    }
}
