use std::collections::VecDeque;

pub fn rotten_oranges(graph: &mut Vec<Vec<i32>>) -> i32 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> =
        vec![vec![false; graph[0].len()]; graph.len()];
    let mut num_level = 0;

    for i in 0..graph.len() {
        for j in 0..graph[0].len() {
            if graph[i][j] == -1 {
                queue.push_back((i, j));
                num_level += 1;
            }
        }
    }

    let mut min = 0;

    while queue.is_empty() != true {
        let mut temp = 0;

        for _ in 0..num_level {
            let curr_node = queue.pop_front().unwrap();

            if curr_node.0 + 1 < graph.len()
                && !visited[curr_node.0 + 1][curr_node.1]
                && graph[curr_node.0 + 1][curr_node.1] == 1
            {
                graph[curr_node.0 + 1][curr_node.1] = -1;
                visited[curr_node.0 + 1][curr_node.1] = true;
                temp += 1;
                queue.push_back((curr_node.0 + 1, curr_node.1));
            }

            if curr_node.1 > 0
                && !visited[curr_node.0][curr_node.1 - 1]
                && graph[curr_node.0][curr_node.1 - 1] == 1
            {
                graph[curr_node.0][curr_node.1 - 1] = -1;
                visited[curr_node.0][curr_node.1 - 1] = true;
                temp += 1;
                queue.push_back((curr_node.0, curr_node.1 - 1));
            }

            if curr_node.0 > 0
                && !visited[curr_node.0 - 1][curr_node.1]
                && graph[curr_node.0 - 1][curr_node.1] == 1
            {
                graph[curr_node.0 - 1][curr_node.1] = -1;
                visited[curr_node.0 - 1][curr_node.1] = true;
                temp += 1;
                queue.push_back((curr_node.0 - 1, curr_node.1));
            }

            if curr_node.1 + 1 < graph[0].len()
                && !visited[curr_node.0][curr_node.1 + 1]
                && graph[curr_node.0][curr_node.1 + 1] == 1
            {
                graph[curr_node.0][curr_node.1 + 1] = -1;
                visited[curr_node.0][curr_node.1 + 1] = true;
                temp += 1;
                queue.push_back((curr_node.0, curr_node.1 + 1));
            }
        }

        min += 1;
        num_level = temp;
    }

    return min-1;
}

fn main() {
    let mut graph = vec![
        vec![-1, 1, 1],
        vec![1, 1, 0],
        vec![0, 1, 1],
    ];


    let result = rotten_oranges(&mut graph);
    println!("Minutes taken: {}", result);
}
