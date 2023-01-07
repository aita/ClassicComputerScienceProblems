use std::{cmp::Reverse, collections::BinaryHeap};

use classic_computer_science_problems::graph::*;

type WeightedPath = Vec<WeightedEdge>;

struct OrdEdge(WeightedEdge);

impl PartialEq for OrdEdge {
    fn eq(&self, other: &Self) -> bool {
        self.0.weight() == other.0.weight()
    }
}

impl Eq for OrdEdge {}

impl PartialOrd for OrdEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.weight().partial_cmp(&other.0.weight())
    }
}

impl Ord for OrdEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn total_weight(wp: &WeightedPath) -> f64 {
    wp.iter().map(|e| e.weight()).sum()
}

fn mst<V: PartialEq + Clone>(wg: &WeightedGraph<V>, start: usize) -> Option<WeightedPath> {
    if start > wg.vertex_count() - 1 {
        return None;
    }
    let mut result = Vec::new();
    let mut heap = BinaryHeap::new();
    let mut visited = vec![false; wg.vertex_count()];

    visited[start] = true;
    for edge in wg.edges_for_index(start) {
        if !visited[edge.v()] {
            heap.push(Reverse(OrdEdge(edge.clone())));
        }
    }
    while let Some(Reverse(OrdEdge(ref edge))) = heap.pop() {
        if visited[edge.v()] {
            continue;
        }
        result.push(edge.clone());
        let index = edge.v();
        visited[index] = true;
        for edge in wg.edges_for_index(index) {
            if !visited[edge.v()] {
                heap.push(Reverse(OrdEdge(edge.clone())));
            }
        }
    }

    Some(result)
}

fn main() {
    let mut city_graph = WeightedGraph::new(vec![
        "Seattle",
        "San Francisco",
        "Los Angeles",
        "Riverside",
        "Phoenix",
        "Chicago",
        "Boston",
        "New York",
        "Atlanta",
        "Miami",
        "Dallas",
        "Houston",
        "Detroit",
        "Philadelphia",
        "Washington",
    ]);

    city_graph.add_edge_by_vertices(&"Seattle", &"Chicago", 1737.0);
    city_graph.add_edge_by_vertices(&"Seattle", &"San Francisco", 678.0);
    city_graph.add_edge_by_vertices(&"San Francisco", &"Riverside", 386.0);
    city_graph.add_edge_by_vertices(&"San Francisco", &"Los Angeles", 348.0);
    city_graph.add_edge_by_vertices(&"Los Angeles", &"Riverside", 50.0);
    city_graph.add_edge_by_vertices(&"Los Angeles", &"Phoenix", 357.0);
    city_graph.add_edge_by_vertices(&"Riverside", &"Phoenix", 307.0);
    city_graph.add_edge_by_vertices(&"Riverside", &"Chicago", 1704.0);
    city_graph.add_edge_by_vertices(&"Phoenix", &"Dallas", 887.0);
    city_graph.add_edge_by_vertices(&"Phoenix", &"Houston", 1015.0);
    city_graph.add_edge_by_vertices(&"Dallas", &"Chicago", 805.0);
    city_graph.add_edge_by_vertices(&"Dallas", &"Atlanta", 721.0);
    city_graph.add_edge_by_vertices(&"Dallas", &"Houston", 225.0);
    city_graph.add_edge_by_vertices(&"Houston", &"Atlanta", 702.0);
    city_graph.add_edge_by_vertices(&"Houston", &"Miami", 968.0);
    city_graph.add_edge_by_vertices(&"Atlanta", &"Chicago", 588.0);
    city_graph.add_edge_by_vertices(&"Atlanta", &"Washington", 543.0);
    city_graph.add_edge_by_vertices(&"Atlanta", &"Miami", 604.0);
    city_graph.add_edge_by_vertices(&"Miami", &"Washington", 923.0);
    city_graph.add_edge_by_vertices(&"Chicago", &"Detroit", 238.0);
    city_graph.add_edge_by_vertices(&"Detroit", &"Boston", 613.0);
    city_graph.add_edge_by_vertices(&"Detroit", &"Washington", 396.0);
    city_graph.add_edge_by_vertices(&"Detroit", &"New York", 482.0);
    city_graph.add_edge_by_vertices(&"Boston", &"New York", 190.0);
    city_graph.add_edge_by_vertices(&"New York", &"Philadelphia", 81.0);
    city_graph.add_edge_by_vertices(&"Philadelphia", &"Washington", 123.0);

    let result = mst(&city_graph, 0);

    if let Some(result) = result {
        for edge in result.iter() {
            println!(
                "{} {} -> {}",
                city_graph.vertex_at(edge.u()),
                edge.weight(),
                city_graph.vertex_at(edge.v()),
            );
        }
        println!("Total weight: {}", total_weight(&result));
    } else {
        println!("No solution found");
    }
}
