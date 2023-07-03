use crate::dijkstra::graph_matrix;
use crate::dijkstra::utils::*;
use crate::graph_type::*;

/// Converts map based graph into vector based graph
pub fn graph_map_to_vector(graph_map: &GraphMap) -> Option<GraphVector> {
    let node_count = graph_map.len();
    let mut vector_graph: GraphVector = GraphVector::with_capacity(node_count);

    for (node, map) in graph_map {
        let mut edges: Vec<(String, u32)> = Vec::with_capacity(map.len());
        for (node, distance) in map {
            edges.push((node.to_string(), *distance));
        }
        vector_graph.push((node.to_string(), edges));
    }

    Some(vector_graph)
}

pub fn graph_vector_to_matrix(graph_vector: &GraphVector) -> (Vec<String>, GraphMatrix) {
    let node_count = graph_vector.len();

    let mut names: Vec<String> = graph_vector.iter().map(|x| x.0.clone()).collect();
    names.sort();

    let name_lookup = name_lookup_map(&names);

    let mut matrix = vec![vec![0; node_count]; node_count];

    for (node1, edges) in graph_vector.iter() {
        let index1 = name_lookup[node1];
        for (node2, distance) in edges.iter() {
            let index2 = name_lookup[node2];
            matrix[index1][index2] = *distance;
            matrix[index2][index1] = *distance;
        }
    }

    (names, matrix)
}

pub fn shortest_path_tree(
    graph_vector: &GraphVector,
    source: &str,
) -> Option<ShortestPathTreeNamed> {
    let (node_names, matrix) = graph_vector_to_matrix(&graph_vector);

    let position = node_names.iter().position(|x| x == source);
    if position.is_none() {
        return None;
    }

    let source_index = position.unwrap();
    let shortest_path_tree_nodes = graph_matrix::shortest_path_tree(&matrix, source_index);
    if shortest_path_tree_nodes.is_none() {
        return None;
    }

    shortest_path_tree_named_from_numbered(&node_names, &shortest_path_tree_nodes.unwrap())
}

pub fn shortest_path_tree_named_from_numbered(
    node_names: &Vec<String>,
    shortest_path_tree: &ShortestPathTree,
) -> Option<ShortestPathTreeNamed> {
    if shortest_path_tree.is_empty() {
        return None;
    }

    let node_count = shortest_path_tree.len();
    let mut tree_nodes_string: Vec<ShortestPathTreeNodeNamed> = Vec::with_capacity(node_count);
    for tree_node in shortest_path_tree {
        tree_nodes_string.push(ShortestPathTreeNodeNamed {
            from: node_names[tree_node.from].clone(),
            to: node_names[tree_node.to].clone(),
            distance: tree_node.distance,
            previous: node_names[tree_node.previous].clone(),
        })
    }

    Some(tree_nodes_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_samples;

    #[test]
    fn graph_vector_sample1_test() {
        graph_vector_test(graph_samples::sample1);
    }

    #[test]
    fn graph_vector_sample2_test() {
        graph_vector_test(graph_samples::sample2);
    }

    #[test]
    fn graph_vector_sample3_test() {
        graph_vector_test(graph_samples::sample3);
    }

    #[test]
    fn graph_vector_sample4_test() {
        graph_vector_test(graph_samples::sample4);
    }

    fn graph_vector_test(fn_test_input: fn() -> (GraphMap, Vec<ShortestPathNamed>)) {
        let (graph_map, expected_shortest_paths) = fn_test_input();
        assert!(!graph_map.is_empty());
        assert!(!expected_shortest_paths.is_empty());
        assert_eq!(expected_shortest_paths.len(), graph_map.len());

        let graph_vector = super::graph_map_to_vector(&graph_map).unwrap();
        assert!(!graph_vector.is_empty());

        let option_shortest_path_tree =
            shortest_path_tree(&graph_vector, &expected_shortest_paths[0].from);
        assert!(option_shortest_path_tree.is_some());

        let shortest_path_tree = option_shortest_path_tree.unwrap();
        let expected_shortest_path_tree =
            build_path_tree_nodes_from_path_named(&expected_shortest_paths);
        assert_eq!(shortest_path_tree, expected_shortest_path_tree);

        for expected_shortest_path in expected_shortest_paths {
            let shortest_path = build_shortest_path_from_tree_named(
                &expected_shortest_path.to,
                &shortest_path_tree,
            );
            assert!(shortest_path.is_some());
            assert_eq!(shortest_path.unwrap(), expected_shortest_path);
        }
    }
}
