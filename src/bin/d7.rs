use petgraph::{
    algo,
    graph::{Graph, NodeIndex},
    visit::EdgeRef,
};
fn dir_sum(graph: &Graph<String, u64>, i: NodeIndex) -> u64 {
    let mut sum = 0;
    for edge in graph.edges(i) {
        sum += edge.weight();
        sum += dir_sum(graph, edge.target());
    }
    sum
}
fn main() {
    // Read input
    let input = include_str!("../../input/d7.txt");
    // Create root directory

    // Create graph
    let mut graph = Graph::<String, u64>::new();
    let root = graph.add_node("/".to_string());
    let mut cursor = root;
    // Read each command and output
    for cmd in input.split("\n$ ").skip(1) {
        let lines = cmd.split("\n").collect::<Vec<&str>>();
        if lines[0].starts_with("ls") {
            for &line in lines[1..].iter() {
                if line.starts_with("dir") {
                    let name = line[4..].to_string();
                    let node = graph.add_node(name);
                    graph.add_edge(cursor, node, 0);
                } else {
                    let parts = line.split(" ").collect::<Vec<&str>>();
                    let size = parts[0].parse::<u64>().unwrap();
                    let name = parts[1].to_string();
                    let node = graph.add_node(name);
                    graph.add_edge(cursor, node, size);
                }
            }
        } else if lines[0].starts_with("cd") {
            let name = lines[0][3..].to_string();
            if name == ".." {
                cursor = graph
                    .neighbors_directed(cursor, petgraph::Incoming)
                    .next()
                    .unwrap();
            } else {
                for edge in graph.edges(cursor) {
                    let node = edge.target();
                    if graph[node] == name {
                        cursor = node;
                        break;
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for edge in graph.edge_references() {
        let source = edge.source();
        let target = edge.target();
        let weight = edge.weight();
        if *weight == 0 {
            let d_sum = dir_sum(&graph, target);
            if d_sum <= 100000 {
                sum += d_sum;
                println!("{} = {}", graph[target], d_sum);
            }
        }
    }
    println!("P1 Sum: {}", sum);
    // Part 2
    let required_space = 30000000 - (70000000 - dir_sum(&graph, root));
    let mut min_val: u64 = 70000000;
    let mut min_node = root;
    for edge in graph.edge_references() {
        let source = edge.source();
        let target = edge.target();
        let weight = edge.weight();
        if *weight == 0 {
            let d_sum = dir_sum(&graph, target);
            if d_sum >= required_space && d_sum < min_val {
                min_val = d_sum;
                min_node = target;
            }
        }
    }
    println!("P2 Sum: {}", min_val);
}
