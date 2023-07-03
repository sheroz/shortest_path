use shortest_path::*;

fn main() {
    // Dijkstra’s Shortest Path Algorithm
    let (graph, _) = graph_samples::sample1();

    let from = "A";
    let to = "I";
    let shortest_path = dijkstra::graph_map::shortest_path(&graph, &from, &to);
    println!("Shortest path from {from} to {to} is :\n {:?}\n", shortest_path);
}