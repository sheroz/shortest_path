use crate::graph_type::*;
use std::collections::HashMap;

pub fn shortest_path_tree(graph_matrix: &GraphMatrix, source: usize) -> Option<Vec<ShortestPathTreeNode>> {
    
    // check if source node exists in the matrix
    let node_count = graph_matrix.len();
    if source >= node_count {
        return None;
    }

    let mut visited = vec![false; node_count];
    let mut shortest_path_tree: Vec<ShortestPathTreeNode> = Vec::with_capacity(node_count);
    for node in 0..node_count {
        shortest_path_tree.push(ShortestPathTreeNode {
            from: source,
            to: node,
            distance: u32::MAX,
            previous: node,
        });
    }

    // starting from the source node
    let mut node = source;
    shortest_path_tree[node].distance = 0;

    while node != usize::MAX {
        let node_distance = shortest_path_tree[node].distance;
        visited[node] = true;

        let column = &graph_matrix[node];

        // applying the core dijkstra algorithm
        // calculating new distance and setting a previous node to follow from
        for index in 0..node_count {
            if !visited[index] {
                let distance = column[index];
                if distance > 0 {
                    let mut record = &mut shortest_path_tree[index];
                    let new_distance = node_distance + distance;
                    if record.distance > new_distance {
                        record.distance = new_distance;
                        record.previous = node;
                    }
                }
            }
        }

        // checking for not visited record with min distance
        node = usize::MAX;
        let mut min_distance = u32::MAX;
        for index in 0..node_count {
            let record = &shortest_path_tree[index];
            if !visited[index] && record.distance < min_distance {
                min_distance = record.distance;
                node = index;
            }
        }
    }

    shortest_path_tree.sort_by(|a, b| a.to.cmp(&b.to));

    Some(shortest_path_tree)
}

pub fn graph_map_to_matrix(graph_map: &GraphMap) -> (Vec<String>, GraphMatrix) {
    let node_count = graph_map.len();

    let mut names: Vec<String> = graph_map.keys().map(|x| x.clone()).collect();
    names.sort();

    let mut name_lookup: HashMap<String, usize> = HashMap::with_capacity(node_count);
    for name in names.iter() {
        name_lookup.insert(name.clone(), name_lookup.len());
    }

    let mut matrix = vec![vec![0; node_count]; node_count];

    for (node1, map) in graph_map {
        let index1 = name_lookup[node1];
        for (node2, distance) in map {
            let index2 = name_lookup[node2];
            matrix[index1][index2] = *distance;
            matrix[index2][index1] = *distance;
        }
    }

    (names, matrix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dijkstra::utils::*;
    use crate::graph_samples;

    #[test]
    fn graph_matrix_sample1_test() {
        graph_matrix_test(graph_samples::sample1);
    }

    #[test]
    fn graph_matrix_sample2_test() {
        graph_matrix_test(graph_samples::sample2);
    }

    #[test]
    fn graph_matrix_sample3_test() {
        graph_matrix_test(graph_samples::sample3);
    }

    #[test]
    fn graph_matrix_sample4_test() {
        graph_matrix_test(graph_samples::sample4);
    }

    fn graph_matrix_test(fn_test_input: fn() -> (GraphMap, Vec<ShortestPathNamed>)) {
        let (graph_map, expected_shortest_paths_named) = fn_test_input();
        assert!(!graph_map.is_empty());
        assert!(!expected_shortest_paths_named.is_empty());

        // convert named vectors into numbered versions
        let (names, graph_matrix) = graph_map_to_matrix(&graph_map);
        assert!(!names.is_empty());
        assert_eq!(names.len(), graph_map.len());

        assert!(!graph_matrix.is_empty());
        assert_eq!(graph_matrix.len(), names.len());

        let option_expected_shortest_paths =
            shortest_paths_from_named(&names, &expected_shortest_paths_named);
        assert!(option_expected_shortest_paths.is_some());
        let expected_shortest_paths = option_expected_shortest_paths.unwrap();
        let expected_shortest_path_tree = build_path_tree_nodes_from_path(&expected_shortest_paths);

        // call the core function
        let option_shortest_path_tree =
            shortest_path_tree(&graph_matrix, expected_shortest_path_tree[0].from);
        assert!(option_shortest_path_tree.is_some());
        let shortest_path_tree = option_shortest_path_tree.unwrap();
        assert_eq!(shortest_path_tree, expected_shortest_path_tree);

        // check builded and expected paths
        for expected_shortest_path in expected_shortest_paths {
            let option_shortest_path =
                build_shortest_path_from_tree(expected_shortest_path.to, &shortest_path_tree);
            assert!(option_shortest_path.is_some());
            let shortest_path = option_shortest_path.unwrap();
            assert_eq!(shortest_path, expected_shortest_path);
        }
    }
}
