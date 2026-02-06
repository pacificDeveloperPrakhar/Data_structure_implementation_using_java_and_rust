use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {

        let mut words = word_list.clone();
        let mut src = usize::MAX;
        let mut tgt = usize::MAX;

        for i in 0..words.len() {
            if words[i] == begin_word {
                src = i;
            }
            if words[i] == end_word {
                tgt = i;
            }
        }

        if tgt == usize::MAX {
            return vec![];
        }

        if src == usize::MAX {
            words.push(begin_word);
            src = words.len() - 1;
        }

        // ===== wildcard graph =====
        let mut graph: HashMap<String, Vec<usize>> = HashMap::new();
        let mut lookup: Vec<Vec<String>> = vec![vec![]; words.len()];

        for i in 0..words.len() {
            let b = words[i].as_bytes();
            for j in 0..b.len() {
                let mut p = b.to_vec();
                p[j] = b'*';
                let s = String::from_utf8(p).unwrap();
                lookup[i].push(s.clone());
                graph.entry(s).or_default().push(i);
            }
        }

        // ===== BFS with parents =====
        let mut dist = vec![-1; words.len()];
        let mut parents: Vec<Vec<usize>> = vec![vec![]; words.len()];
        let mut q = VecDeque::new();

        dist[src] = 0;
        q.push_back(src);

        let mut found = false;

        while !q.is_empty() && !found {
            let size = q.len();
            for _ in 0..size {
                let u = q.pop_front().unwrap();

                for pat in &lookup[u] {
                    for &v in &graph[pat] {
                        if dist[v] == -1 {
                            dist[v] = dist[u] + 1;
                            parents[v].push(u);
                            if v == tgt {
                                found = true;
                            }
                            q.push_back(v);
                        } else if dist[v] == dist[u] + 1 {
                            parents[v].push(u);
                        }
                    }
                }
            }
        }

        if dist[tgt] == -1 {
            return vec![];
        }

        // ===== backtracking =====
        let mut ans = vec![];
        let mut path = vec![tgt];

        Self::backtrack(
            tgt,
            src,
            &parents,
            &words,
            &mut path,
            &mut ans,
        );

        ans
    }

    fn backtrack(
        u: usize,
        src: usize,
        parents: &Vec<Vec<usize>>,
        words: &Vec<String>,
        path: &mut Vec<usize>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if u == src {
            let mut seq = path.iter().rev().map(|&i| words[i].clone()).collect::<Vec<_>>();
            ans.push(seq);
            return;
        }

        for &p in &parents[u] {
            path.push(p);
            Self::backtrack(p, src, parents, words, path, ans);
            path.pop();
        }
    }
}
