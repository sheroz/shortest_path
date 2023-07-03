use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub type GraphMap = HashMap<String, HashMap<String, u32>>;
pub type GraphVector = Vec<(String, Vec<(String, u32)>)>;
pub type GraphMatrix = Vec<Vec<u32>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathTreeNode {
    pub from: usize,
    pub to: usize,
    pub distance: u32,
    pub previous: usize,
}

pub type ShortestPathTree = Vec<ShortestPathTreeNode>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPath {
    pub from: usize,
    pub to: usize,
    pub distance: u32,
    pub path: Vec<usize>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathTreeNodeNamed {
    pub from: String,
    pub to: String,
    pub distance: u32,
    pub previous: String,
}

pub type ShortestPathTreeNamed = Vec<ShortestPathTreeNodeNamed>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortestPathNamed {
    pub from: String,
    pub to: String,
    pub distance: u32,
    pub path: Vec<String>,
}
