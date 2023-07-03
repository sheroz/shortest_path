// Implements Dijkstra’s Shortest Path Algorithm
// Useful resources:
// https://ieeexplore.ieee.org/document/9190342
// https://www.researchgate.net/publication/348997309_Greedy_A-Star_and_Dijkstra's_Algorithms_in_Finding_Shortest_Path
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
// https://www.youtube.com/watch?v=pVfj6mxhdMw
// https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/

use crate::graph_type::*;
use crate::dijkstra::utils::*;

pub fn shortest_path_tree(graph_map: &GraphMap, source: &str) -> Option<ShortestPathTreeNamed> {
    struct PathRecord {
        name: String,
        distance: Option<u32>,
        prev_node: String,
        visited: bool,
    }

    // check if from and to nodes exists in graph
    if graph_map.get(source).is_none() {
        return None;
    }

    let mut path_table: Vec<PathRecord> = Vec::new();
    for node in graph_map.keys() {
        let path_record = PathRecord {
            name: node.to_string(),
            distance: None,
            prev_node: node.to_string(),
            visited: false,
        };
        path_table.push(path_record);
    }

    // make starting node distance as zero
    path_table
        .iter_mut()
        .find(|r| r.name == source)
        .unwrap()
        .distance = Some(0);

    loop {
        // check for min distance and not visited
        let mut records: Vec<_> = path_table
            .iter_mut()
            .filter(|r| !r.visited && r.distance.is_some())
            .collect();
        if records.is_empty() {
            break;
        }

        records.sort_by(|a, b| b.distance.cmp(&a.distance));

        let mut cur_record = records.pop().unwrap();

        let node_name = cur_record.name.clone();
        let node_distance = cur_record.distance.unwrap();
        cur_record.visited = true;

        let node = graph_map.get(&node_name);
        if node.is_none() {
            panic!("The node {} does not exists in the given graph!", node_name);
        }

        let edges = node.unwrap();
        for (edge_name, edge_distance) in edges {
            let path_record = path_table
                .iter_mut()
                .find(|r| !r.visited && r.name == *edge_name);
            if path_record.is_some() {
                let record = path_record.unwrap();
                if record.distance.is_none()
                    || record.distance.unwrap() > (edge_distance + node_distance)
                {
                    record.distance = Some(edge_distance + node_distance);
                    record.prev_node = node_name.clone();
                }
            }
        }
    }

    let mut edges: Vec<ShortestPathTreeNodeNamed> = Vec::new();
    for record in path_table {
        let edge = ShortestPathTreeNodeNamed {
            from: source.to_string(),
            to: record.name.clone(),
            previous: record.prev_node.clone(),
            distance: record.distance.unwrap(),
        };
        edges.push(edge);
    }
    edges.sort_by(|a, b| a.to.cmp(&b.to));

    Some(edges)
}

pub fn shortest_path(graph_map: &GraphMap, from: &str, to: &str) -> Option<ShortestPathNamed> {
    if graph_map.get(from).is_none() || graph_map.get(to).is_none() {
        return None;
    }

    let shortest_path_tree = shortest_path_tree(&graph_map, &from);
    if shortest_path_tree.is_none() {
        return None;
    }

    build_shortest_path_from_tree_named(&to, &shortest_path_tree.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_samples;

    #[test]
    fn graph_map_sample1_test() {
        graph_map_test(graph_samples::sample1)
    }
    #[test]
    fn graph_map_sample2_test() {
        graph_map_test(graph_samples::sample2)
    }
    #[test]
    fn graph_map_sample3_test() {
        graph_map_test(graph_samples::sample3)
    }

    #[test]
    fn graph_map_sample4_test() {
        graph_map_test(graph_samples::sample4)
    }

    fn graph_map_test(fn_test_input: fn() -> (GraphMap, Vec<ShortestPathNamed>)) {
        let (graph_map, expected_shortest_paths) = fn_test_input();
        assert!(!graph_map.is_empty());
        assert!(!expected_shortest_paths.is_empty());
        assert_eq!(expected_shortest_paths.len(), graph_map.len());

        let option_shortest_path_tree =
            shortest_path_tree(&graph_map, &expected_shortest_paths[0].from);
        assert!(option_shortest_path_tree.is_some());

        let shortest_path_tree = option_shortest_path_tree.unwrap();
        let expected_shortest_path_tree = build_path_tree_nodes_from_path_named(&expected_shortest_paths);
        assert_eq!(shortest_path_tree, expected_shortest_path_tree);

        for expected_shortest_path in expected_shortest_paths {
            let shortest_path =
                build_shortest_path_from_tree_named(&expected_shortest_path.to, &shortest_path_tree);
            assert!(shortest_path.is_some());
            assert_eq!(shortest_path.unwrap(), expected_shortest_path);
        }
    }

    #[test]
    fn shortest_path_test() {
        let (graph, _) = graph_samples::sample1();
        assert!(!graph.is_empty());

        let from = "A";

        let mut to = "D";
        let shortest_path_result = shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path_result.unwrap(),
            ShortestPathNamed {
                from: from.to_string(),
                to: to.to_string(),
                distance: 19,
                path: ["A", "B", "C", "D"].iter().map(|n| n.to_string()).collect()
            }
        );

        to = "E";
        let shortest_path_result = shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path_result.unwrap(),
            ShortestPathNamed {
                from: from.to_string(),
                to: to.to_string(),
                distance: 21,
                path: ["A", "H", "G", "F", "E"]
                    .iter()
                    .map(|n| n.to_string())
                    .collect()
            }
        );

        to = "I";
        let shortest_path_result = shortest_path(&graph, &from, &to);
        assert_eq!(
            shortest_path_result.unwrap(),
            ShortestPathNamed {
                from: from.to_string(),
                to: to.to_string(),
                distance: 14,
                path: ["A", "B", "C", "I"].iter().map(|n| n.to_string()).collect()
            }
        );
    }
}
