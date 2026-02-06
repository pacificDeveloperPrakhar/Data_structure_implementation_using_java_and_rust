use std::collections::{VecDeque, HashMap};
pub fn word_ladder(word_lis:&Vec<String>,begin_word:String,end_word:String)->i32
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
        return -1;
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
    
    let mut queue:VecDeque<usize>=VecDeque::new();
    let mut visited:Vec<bool>=vec![false;word_lis.len()];
    queue.push_back(src);
    visited[src]=true;
    let mut num_level=1;
    let mut changes=0;
    while queue.is_empty()!=true
    {
        let mut temp=0;
        for i in 0..num_level
        {
         let curr=queue.pop_front().unwrap();
         if curr==tgt
         {
            return changes;
         }
         let keys=lookup.get(&curr).unwrap();
         for j in 0..keys.len()
         {
          let neigh=graph.get(&keys[j]).unwrap();
          for k in 0..neigh.len()
          {
            if visited[neigh[k]]!=true
            {
                queue.push_back(neigh[k]);
                visited[neigh[k]]=true;
                temp+=1;
            }

          }
         }
        }
        num_level=temp;
        changes+=1;
    }
    return changes;
}

pub fn main() {
    // Test case 1: classic example
    let words1 = vec![
        "hit".to_string(),
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
    ];

    let res1 = word_ladder(&words1, "hit".to_string(), "cog".to_string());
    println!("Test 1 result: {}", res1); // expected: 4 or 5 depending on definition


    // Test case 2: begin word not in list
    let words2 = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
    ];

    let res2 = word_ladder(&words2, "hit".to_string(), "cog".to_string());
    println!("Test 2 result: {}", res2);


    // Test case 3: end word missing
    let words3 = vec![
        "hit".to_string(),
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
    ];

    let res3 = word_ladder(&words3, "hit".to_string(), "cog".to_string());
    println!("Test 3 result: {}", res3); // expected: -1


    // Test case 4: begin == end
    let words4 = vec![
        "same".to_string(),
        "came".to_string(),
        "lame".to_string(),
    ];

    let res4 = word_ladder(&words4, "same".to_string(), "same".to_string());
    println!("Test 4 result: {}", res4);


    // Test case 5: no possible transformation
    let words5 = vec![
        "aaa".to_string(),
        "bbb".to_string(),
        "ccc".to_string(),
    ];

    let res5 = word_ladder(&words5, "aaa".to_string(), "ccc".to_string());
    println!("Test 5 result: {}", res5);
}
