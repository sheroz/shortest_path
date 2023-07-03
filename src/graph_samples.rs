use crate::graph_type::*;
use serde_json::json;
use std::collections::HashMap;

pub fn sample1() -> (GraphMap, Vec<ShortestPathNamed>) {
    let input_graph_edges = [
        ("A", "B", 4),
        ("A", "H", 8),
        ("B", "C", 8),
        ("B", "H", 11),
        ("C", "D", 7),
        ("C", "I", 2),
        ("C", "F", 3),
        ("D", "E", 9),
        ("D", "F", 14),
        ("E", "F", 10),
        ("F", "G", 2),
        ("G", "H", 1),
        ("G", "I", 6),
        ("H", "I", 7),
    ];

    let mut graph = GraphMap::new();
    input_graph_edges
        .iter()
        .for_each(|x| add_edge_from_named_node(&mut graph, x.0, x.1, x.2));

    /*
    Graph
        A   B   C   D   E   F   G   H   I
    --------------------------------------
    A | 0,  4,  0,  0,  0,  0,  0,  8,  0
    B | 4,  0,  8,  0,  0,  0,  0, 11,  0
    C | 0,  8,  0,  7,  0,  4,  0,  0,  2
    D | 0,  0,  7,  0,  9, 14,  0,  0,  0
    E | 0,  0,  0,  9,  0, 10,  0,  0,  0
    F | 0,  0,  4, 14, 10,  0,  2,  0,  0
    G | 0,  0,  0,  0,  0,  2,  0,  1,  6
    H | 8, 11,  0,  0,  0,  0,  1,  0,  7
    I | 0,  0,  2,  0,  0,  0,  6,  7,  0

    Shortest-path tree:
    "B"  4 "A"
    "C" 12 "B"
    "D" 19 "C"
    "E" 21 "F"
    "F" 11 "G"
    "G"  9 "H"
    "H"  8 "A"
    "I" 14 "C"

    Minimum distance from A:
    B = 4,  A->B
    C = 12, A->B->C
    D = 19, A->B->C->D
    E = 21, A->H->G->F->E
    F = 11, A->H->G->F
    G = 9,  A->H->G
    H = 8,  A->H
    I = 14, A->B->C->I
    */

    let input_shortest_paths = [
        ("A", "A", 0, vec!["A", "A"]),
        ("A", "B", 4, vec!["A", "B"]),
        ("A", "C", 12, vec!["A", "B", "C"]),
        ("A", "D", 19, vec!["A", "B", "C", "D"]),
        ("A", "E", 21, vec!["A", "H", "G", "F", "E"]),
        ("A", "F", 11, vec!["A", "H", "G", "F"]),
        ("A", "G", 9, vec!["A", "H", "G"]),
        ("A", "H", 8, vec!["A", "H"]),
        ("A", "I", 14, vec!["A", "B", "C", "I"]),
    ];

    let mut expected_shortest_paths: Vec<ShortestPathNamed> =
        Vec::with_capacity(input_shortest_paths.len());
    input_shortest_paths.iter().for_each(|x| {
        expected_shortest_paths.push(ShortestPathNamed {
            from: x.0.to_string(),
            to: x.1.to_string(),
            distance: x.2,
            path: x.3.iter().map(|v| v.to_string()).collect(),
        })
    });
    (graph, expected_shortest_paths)
}

pub fn sample2() -> (GraphMap, Vec<ShortestPathNamed>) {
    let input_graph_edges = [
        ("A", "B", 6),
        ("A", "D", 1),
        ("B", "D", 2),
        ("B", "E", 2),
        ("B", "C", 5),
        ("C", "E", 5),
        ("D", "E", 1),
    ];

    let mut graph = GraphMap::new();
    input_graph_edges
        .iter()
        .for_each(|x| add_edge_from_named_node(&mut graph, x.0, x.1, x.2));

    /*
    Shortest-path tree:
    A 0
    B 3 D
    C 7 E
    D 1 A
    E 2 D

    B = 3, A->D->B
    C = 7, A->D->E->C
    D = 1, A->D
    E = 2, A->D->E
    */

    let input_shortest_paths = r#"
        [
            {"from": "A", "to": "A", "distance": 0, "path": ["A", "A"]},
            {"from": "A", "to": "B", "distance": 3, "path": ["A", "D", "B"]},
            {"from": "A", "to": "C", "distance": 7, "path": ["A", "D", "E", "C"]},
            {"from": "A", "to": "D", "distance": 1, "path": ["A", "D"]},
            {"from": "A", "to": "E", "distance": 2, "path": ["A", "D", "E"]}
        ]
    "#;

    let expected_shortest_paths: Vec<ShortestPathNamed> =
        serde_json::from_str(&input_shortest_paths).unwrap();

    (graph, expected_shortest_paths)
}

pub fn sample3() -> (GraphMap, Vec<ShortestPathNamed>) {
    let input_graph_edges = [
        ("A", "B", 4),
        ("A", "C", 5),
        ("B", "C", 11),
        ("B", "D", 9),
        ("B", "E", 7),
        ("C", "E", 3),
        ("D", "E", 13),
        ("D", "F", 2),
        ("E", "F", 6),
    ];

    let mut graph = GraphMap::new();
    input_graph_edges
        .iter()
        .for_each(|x| add_edge_from_named_node(&mut graph, x.0, x.1, x.2));

    /*
    Shortest-path tree:
    A  0
    B  4 A
    C  5 A
    D 13 B
    E  8 C
    F 14 E

    A = 0
    B = 4,  A−>B
    C = 5,  A−>C
    D = 13, A−>B−>D
    E = 8,  A−>C−>E
    F = 14, A−>C−>E−>F
    */

    let input_shortest_paths = json!([
        {"from": "A", "to": "A", "distance": 0, "path": ["A", "A"]},
        {"from": "A", "to": "B", "distance": 4, "path": ["A", "B"]},
        {"from": "A", "to": "C", "distance": 5, "path": ["A", "C"]},
        {"from": "A", "to": "D", "distance": 13,"path": ["A", "B", "D"]},
        {"from": "A", "to": "E", "distance": 8, "path": ["A", "C", "E"]},
        {"from": "A", "to": "F", "distance": 14,"path": ["A", "C", "E", "F"]}
    ]);

    let expected_shortest_paths: Vec<ShortestPathNamed> =
        serde_json::from_value(input_shortest_paths).unwrap();

    (graph, expected_shortest_paths)
}

pub fn sample4() -> (GraphMap, Vec<ShortestPathNamed>) {
    let input_graph_edges = [
        ("A", "B", 7),
        ("A", "C", 9),
        ("A", "F", 14),
        ("B", "C", 10),
        ("B", "D", 15),
        ("C", "D", 11),
        ("C", "F", 2),
        ("D", "E", 6),
        ("E", "F", 9),
    ];

    let mut graph = GraphMap::new();
    input_graph_edges
        .iter()
        .for_each(|x| add_edge_from_named_node(&mut graph, x.0, x.1, x.2));

    let input_shortest_paths = json!([
        {"from": "A", "to": "A", "distance": 0,  "path": ["A", "A"]},
        {"from": "A", "to": "B", "distance": 7,  "path": ["A", "B"]},
        {"from": "A", "to": "C", "distance": 9,  "path": ["A", "C"]},
        {"from": "A", "to": "D", "distance": 20, "path": ["A", "C", "D"]},
        {"from": "A", "to": "E", "distance": 20, "path": ["A", "C", "F", "E"]},
        {"from": "A", "to": "F", "distance": 11, "path": ["A", "C", "F"]}
    ]);

    let expected_shortest_paths: Vec<ShortestPathNamed> =
        serde_json::from_value(input_shortest_paths).unwrap();

    (graph, expected_shortest_paths)
}

fn add_edge_from_named_node(graph: &mut GraphMap, a: &str, b: &str, distance: u32) {
    let a_node = graph.get_mut(a);
    if a_node.is_some() {
        a_node.unwrap().insert(b.to_string(), distance);
    } else {
        let mut edges = HashMap::new();
        edges.insert(b.to_string(), distance);
        graph.insert(a.to_string(), edges);
    }

    let b_node = graph.get_mut(b);
    if b_node.is_some() {
        b_node.unwrap().insert(a.to_string(), distance);
    } else {
        let mut edges = HashMap::new();
        edges.insert(a.to_string(), distance);
        graph.insert(b.to_string(), edges);
    }
}
