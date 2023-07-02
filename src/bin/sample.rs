use shortest_path::dijkstra;

fn main() {
    // Dijkstra’s Shortest Path Algorithm
    let (graph, _) = dijkstra::test_sample1();

    let from = "A";
    let to = "I";
    let shortest_path = dijkstra::shortest_path_from_map_graph(&graph, &from, &to);
    println!("Shortest path from {from} to {to} is :\n {:?}\n", shortest_path);
}