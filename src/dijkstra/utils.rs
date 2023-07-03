use std::collections::HashMap;
use crate::graph_type::*;

pub fn name_lookup_map(names: &Vec<String>) -> HashMap<String, usize> {
    let mut name_lookup: HashMap<String, usize> = HashMap::with_capacity(names.len());
    for name in names {
        name_lookup.insert(name.clone(), name_lookup.len());
    }
    name_lookup
}

pub fn shortest_paths_from_named(node_names: &Vec<String>, expected_shortest_paths_named: &Vec<ShortestPathNamed>) -> Option<Vec<ShortestPath>> {
    if node_names.is_empty() || expected_shortest_paths_named.is_empty() {
        return None
    }

    let name_lookup = name_lookup_map(node_names);

    let mut expected_shortest_paths: Vec<ShortestPath> = Vec::with_capacity(expected_shortest_paths_named.len());
    for named_path in expected_shortest_paths_named {
        expected_shortest_paths.push(
            ShortestPath {
                from: name_lookup[&named_path.from],
                to: name_lookup[&named_path.to], 
                distance: named_path.distance,
                path: named_path.path.iter().map(|x| name_lookup[x]).collect()
            }
        )
    }

    Some(expected_shortest_paths)
}

pub fn build_shortest_path_from_tree_named(
    to: &str,
    shortest_path_tree: &ShortestPathTreeNamed,
) -> Option<ShortestPathNamed> {
    let mut path: Vec<String> = Vec::new();
    let nodes = &shortest_path_tree;
    let node = nodes.iter().find(|e| e.to == to).unwrap();
    let from = node.from.clone();
    let distance = node.distance;
    let mut previous = node.previous.clone();

    path.push(to.to_string());
    while previous != from {
        path.push(previous.clone());

        let path_record_option = nodes.iter().find(|e| e.to == previous);
        if path_record_option.is_none() {
            panic!("Previous node not found: {}", previous);
        }

        previous = path_record_option.unwrap().previous.clone();
    }

    path.push(from.to_string());
    path.reverse();

    Some(ShortestPathNamed {
        from: from.to_string(),
        to: to.to_string(),
        distance,
        path,
    })
}

pub fn build_shortest_path_from_tree(to: usize, shortest_path_tree: &Vec<ShortestPathTreeNode>) -> Option<ShortestPath> {
    let mut path: Vec<usize> = Vec::new();
    let nodes = &shortest_path_tree;
    let node = nodes.iter().find(|e| e.to == to).unwrap();
    let from = node.from;
    let distance = node.distance;
    let mut previous = node.previous;

    path.push(to);
    while previous != from {
        path.push(previous);

        let path_record_option = nodes.iter().find(|e| e.to == previous);
        if path_record_option.is_none() {
            panic!("Previous node not found: {}", previous);
        }

        previous = path_record_option.unwrap().previous;
    }

    path.push(from);
    path.reverse();

    Some(ShortestPath {
        from,
        to,
        distance,
        path,
    })
}

pub fn shortest_path_tree_from_named(node_names: &Vec<String>, shortest_path_tree_string: &Vec<ShortestPathTreeNodeNamed>) -> Option<ShortestPathTree> {
    if node_names.is_empty() || shortest_path_tree_string.is_empty() {
        return None;
    }

    let name_lookup = name_lookup_map(&node_names);

    let node_count = shortest_path_tree_string.len();
    let mut tree_nodes: Vec<ShortestPathTreeNode> = Vec::with_capacity(node_count);
    for tree_node_string in shortest_path_tree_string {
        tree_nodes.push(
            ShortestPathTreeNode {
                from: name_lookup[&tree_node_string.from],
                to: name_lookup[&tree_node_string.to],
                distance: tree_node_string.distance,
                previous: name_lookup[&tree_node_string.previous]
            }
        )
    }

    Some(tree_nodes)
}

pub fn build_path_tree_nodes_from_path(shortest_paths: &Vec<ShortestPath>) -> Vec<ShortestPathTreeNode> {
    let mut expected_tree_nodes: Vec<ShortestPathTreeNode> = Vec::with_capacity(shortest_paths.len());
    for shortest_path in shortest_paths.iter() {
        expected_tree_nodes.push(
            ShortestPathTreeNode {
                from: shortest_path.from,
                to: shortest_path.to,
                distance: shortest_path.distance,
                previous: shortest_path.path[shortest_path.path.len()-2]
            }
        );     
    }
    expected_tree_nodes
}

pub fn build_path_tree_nodes_from_path_named(shortest_paths: &Vec<ShortestPathNamed>) -> Vec<ShortestPathTreeNodeNamed> {
    let mut expected_tree_nodes: Vec<ShortestPathTreeNodeNamed> = Vec::with_capacity(shortest_paths.len());
    for shortest_path in shortest_paths.iter() {
        expected_tree_nodes.push(
            ShortestPathTreeNodeNamed {
                from: shortest_path.from.to_string(),
                to: shortest_path.to.to_string(),
                distance: shortest_path.distance,
                previous: shortest_path.path[shortest_path.path.len()-2].to_string()
            }
        );     
    }
    expected_tree_nodes
}
