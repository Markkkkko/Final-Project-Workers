

use crate::graph::{EmployeeGraph}; //use crate
use std::collections::HashMap;

pub fn degree_centrality(graph: &EmployeeGraph) -> HashMap<u32, f32> { //create hasmap
    let mut centrality: HashMap<u32, f32> = HashMap::new();

    for node in graph.employees.keys() {//iterate over the keys of the data, the values in there
        let degree = graph.connections.get(node).map(|neighbors| neighbors.len() as f32).unwrap_or(0.0); //unwrap to make readable
        centrality.insert(*node, degree);
    }

    centrality
}

pub fn get_k_representatives(graph: &EmployeeGraph, k: usize) -> Vec<u32> { 
    let centrality = degree_centrality(graph); //central

    let mut sorted_by_centrality: Vec<(u32, f32)> = centrality.into_iter().collect(); //.collect to collect graphs
    sorted_by_centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); //unwrap to make readable
  
    sorted_by_centrality.into_iter().take(k).map(|(id, _)| id).collect() //collect 
}

