pub fn traverse(curr_node: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
    for i in 0..graph.len() {
        if !visited[i] && graph[curr_node][i] == 1 {
            visited[i] = true;
            traverse(i, graph, visited);
        }
    }
}

pub fn count_province(graph: &Vec<Vec<i32>>) -> i32 {
    let mut num_province = 0;
    let mut visited: Vec<bool> = vec![false; graph.len()];

    for i in 0..graph.len() {
        if !visited[i] {
            visited[i] = true;
            traverse(i, graph, &mut visited);
            num_province += 1;
        }
    }

    num_province
}

fn main() {
    let graph = vec![
        vec![1, 1, 0],
        vec![1, 1, 0],
        vec![0, 0, 1],
    ];

    let result = count_province(&graph);
    println!("Number of provinces: {}", result);
}
