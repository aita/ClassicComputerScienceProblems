use std::hash::Hash;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use classic_computer_science_problems::graph::*;

type WeightedPath = Vec<WeightedEdge>;

#[derive(Debug, Clone, PartialEq)]
struct DijkstraNode {
    vertex: usize,
    distance: f64,
}

impl DijkstraNode {
    fn new(vertex: usize, distance: f64) -> Self {
        Self { vertex, distance }
    }
}

impl Eq for DijkstraNode {}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn dijkstra<V: PartialEq + Clone>(
    wg: &WeightedGraph<V>,
    root: V,
) -> (Vec<Option<f64>>, HashMap<usize, WeightedEdge>) {
    let first = wg.index_of(&root);
    let mut distances = vec![None; wg.vertex_count()];
    distances[first] = Some(0.0);
    let mut path_dict = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(DijkstraNode::new(first, 0.0));

    while let Some(DijkstraNode { vertex, .. }) = heap.pop() {
        let u = vertex;
        let dist_u = distances[u].unwrap();
        for we in wg.edges_for_index(u) {
            let dist_v = distances[we.v()];
            if dist_v.is_none() || dist_v.unwrap() > dist_u + we.weight() {
                distances[we.v()] = Some(dist_u + we.weight());
                path_dict.insert(we.v(), we.clone());
                heap.push(DijkstraNode::new(we.v(), dist_u + we.weight()));
            }
        }
    }

    (distances, path_dict)
}

fn distance_array_to_vertex_dict<V: PartialEq + Clone + Hash + Eq>(
    wg: &WeightedGraph<V>,
    distances: &Vec<Option<f64>>,
) -> HashMap<V, Option<f64>> {
    let mut result = HashMap::new();
    for i in 0..distances.len() {
        result.insert(wg.vertex_at(i).clone(), distances[i]);
    }
    result
}

fn path_dict_to_path(
    start: usize,
    end: usize,
    path_dict: &HashMap<usize, WeightedEdge>,
) -> WeightedPath {
    if path_dict.is_empty() {
        return Vec::new();
    }
    let mut edge_path = Vec::new();
    let mut e = &path_dict[&end];
    edge_path.push(e.clone());
    while e.u() != start {
        e = &path_dict[&e.u()];
        edge_path.push(e.clone());
    }
    edge_path.reverse();
    edge_path
}

fn total_weight(wp: &WeightedPath) -> f64 {
    wp.iter().map(|e| e.weight()).sum()
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

    let (distances, path_dict) = dijkstra(&city_graph, "Los Angeles");
    let name_distance = distance_array_to_vertex_dict(&city_graph, &distances);
    println!("Distances from Los Angeles:");
    for (key, value) in name_distance.iter() {
        println!("{} : {}", key, value.unwrap());
    }
    println!();

    println!("Shortest path from Los Angeles to Boston:");
    let path = path_dict_to_path(
        city_graph.index_of(&"Los Angeles"),
        city_graph.index_of(&"Boston"),
        &path_dict,
    );
    for edge in path.iter() {
        println!(
            "{} {} -> {}",
            city_graph.vertex_at(edge.u()),
            edge.weight(),
            city_graph.vertex_at(edge.v()),
        );
    }
    println!("Total weight: {}", total_weight(&path));
}
