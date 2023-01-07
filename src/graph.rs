#[derive(Debug, Clone, PartialEq)]
pub struct Edge(usize, usize);

impl Edge {
    pub fn new(a: usize, b: usize) -> Self {
        Edge(a, b)
    }

    pub fn reversed(&self) -> Self {
        Edge(self.1, self.0)
    }

    pub fn u(&self) -> usize {
        self.0
    }

    pub fn v(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Clone)]
pub struct Graph<Vertex> {
    vertices: Vec<Vertex>,
    edges: Vec<Vec<Edge>>,
}

impl<Vertex> Graph<Vertex>
where
    Vertex: PartialEq + Clone,
{
    pub fn new(vertices: Vec<Vertex>) -> Self {
        let edges = vec![Vec::new(); vertices.len()];
        Graph { vertices, edges }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn edges(&self) -> &Vec<Vec<Edge>> {
        &self.edges
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.iter().map(|e| e.len()).sum()
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        self.vertices.push(vertex);
        self.edges.push(Vec::new());
        self.vertices.len() - 1
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges[edge.0].push(edge.clone());
        self.edges[edge.1].push(edge.reversed());
    }

    pub fn add_edge_by_indices(&mut self, u: usize, v: usize) {
        self.add_edge(Edge::new(u, v));
    }

    pub fn add_edge_by_vertices(&mut self, first: &Vertex, second: &Vertex) {
        let u = self.vertices.iter().position(|v| v == first).unwrap();
        let v = self.vertices.iter().position(|v| v == second).unwrap();
        self.add_edge_by_indices(u, v);
    }

    pub fn vertex_at(&self, index: usize) -> &Vertex {
        &self.vertices[index]
    }

    pub fn index_of(&self, vertex: &Vertex) -> usize {
        self.vertices.iter().position(|v| v == vertex).unwrap()
    }

    pub fn neighbors_for_index(&self, index: usize) -> Vec<Vertex> {
        self.edges[index]
            .iter()
            .map(|e| self.vertex_at(e.1).clone())
            .collect()
    }

    pub fn neighbors_for_vertex(&self, vertex: &Vertex) -> Vec<Vertex> {
        self.neighbors_for_index(self.index_of(vertex))
    }

    pub fn edges_for_index(&self, index: usize) -> Vec<&Edge> {
        self.edges[index].iter().collect()
    }

    pub fn edges_for_vertex(&self, vertex: &Vertex) -> Vec<&Edge> {
        self.edges_for_index(self.index_of(vertex))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeightedEdge(usize, usize, f64);

impl WeightedEdge {
    pub fn new(a: usize, b: usize, weight: f64) -> Self {
        WeightedEdge(a, b, weight)
    }

    pub fn reversed(&self) -> Self {
        WeightedEdge(self.1, self.0, self.2)
    }

    pub fn u(&self) -> usize {
        self.0
    }

    pub fn v(&self) -> usize {
        self.1
    }

    pub fn weight(&self) -> f64 {
        self.2
    }
}



pub struct WeightedGraph<Vertex> {
    vertices: Vec<Vertex>,
    edges: Vec<Vec<WeightedEdge>>,
}

impl<Vertex> WeightedGraph<Vertex>
where
    Vertex: PartialEq + Clone,
{
    pub fn new(vertices: Vec<Vertex>) -> Self {
        let edges = vec![Vec::new(); vertices.len()];
        Self { vertices, edges }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn edges(&self) -> &Vec<Vec<WeightedEdge>> {
        &self.edges
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.iter().map(|e| e.len()).sum()
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        self.vertices.push(vertex);
        self.edges.push(Vec::new());
        self.vertices.len() - 1
    }

    pub fn add_edge(&mut self, edge: WeightedEdge) {
        self.edges[edge.0].push(edge.clone());
        self.edges[edge.1].push(edge.reversed());
    }

    pub fn add_edge_by_indices(&mut self, u: usize, v: usize, weight: f64) {
        self.add_edge(WeightedEdge::new(u, v, weight));
    }

    pub fn add_edge_by_vertices(&mut self, first: &Vertex, second: &Vertex, weight: f64) {
        let u = self.vertices.iter().position(|v| v == first).unwrap();
        let v = self.vertices.iter().position(|v| v == second).unwrap();
        self.add_edge_by_indices(u, v, weight);
    }

    pub fn vertex_at(&self, index: usize) -> &Vertex {
        &self.vertices[index]
    }

    pub fn index_of(&self, vertex: &Vertex) -> usize {
        self.vertices.iter().position(|v| v == vertex).unwrap()
    }

    pub fn neighbors_for_index(&self, index: usize) -> Vec<Vertex> {
        self.edges[index]
            .iter()
            .map(|e| self.vertex_at(e.1).clone())
            .collect()
    }

    pub fn neighbors_for_index_with_weights(&self, index: usize) -> Vec<(Vertex, f64)> {
        self.edges[index]
            .iter()
            .map(|e| (self.vertex_at(e.1).clone(), e.2))
            .collect()
    }

    pub fn neighbors_for_vertex(&self, vertex: &Vertex) -> Vec<Vertex> {
        self.neighbors_for_index(self.index_of(vertex))
    }

    pub fn edges_for_index(&self, index: usize) -> Vec<&WeightedEdge> {
        self.edges[index].iter().collect()
    }

    pub fn edges_for_vertex(&self, vertex: &Vertex) -> Vec<&WeightedEdge> {
        self.edges_for_index(self.index_of(vertex))
    }
}

#[cfg(test)]
mod tests {
    use crate::generic_search::{bfs, node_to_path, Arena};

    use super::*;

    #[test]
    fn test_graph() {
        let mut city_graph = Graph::new(vec![
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
        city_graph.add_edge_by_vertices(&"Seattle", &"Chicago");
        city_graph.add_edge_by_vertices(&"Seattle", &"San Francisco");
        city_graph.add_edge_by_vertices(&"San Francisco", &"Riverside");
        city_graph.add_edge_by_vertices(&"San Francisco", &"Los Angeles");
        city_graph.add_edge_by_vertices(&"Los Angeles", &"Riverside");
        city_graph.add_edge_by_vertices(&"Los Angeles", &"Phoenix");
        city_graph.add_edge_by_vertices(&"Riverside", &"Phoenix");
        city_graph.add_edge_by_vertices(&"Riverside", &"Chicago");
        city_graph.add_edge_by_vertices(&"Phoenix", &"Dallas");
        city_graph.add_edge_by_vertices(&"Phoenix", &"Houston");
        city_graph.add_edge_by_vertices(&"Dallas", &"Chicago");
        city_graph.add_edge_by_vertices(&"Dallas", &"Atlanta");
        city_graph.add_edge_by_vertices(&"Dallas", &"Houston");
        city_graph.add_edge_by_vertices(&"Houston", &"Atlanta");
        city_graph.add_edge_by_vertices(&"Houston", &"Miami");
        city_graph.add_edge_by_vertices(&"Atlanta", &"Chicago");
        city_graph.add_edge_by_vertices(&"Atlanta", &"Washington");
        city_graph.add_edge_by_vertices(&"Atlanta", &"Miami");
        city_graph.add_edge_by_vertices(&"Miami", &"Washington");
        city_graph.add_edge_by_vertices(&"Chicago", &"Detroit");
        city_graph.add_edge_by_vertices(&"Detroit", &"Boston");
        city_graph.add_edge_by_vertices(&"Detroit", &"Washington");
        city_graph.add_edge_by_vertices(&"Detroit", &"New York");
        city_graph.add_edge_by_vertices(&"Boston", &"New York");
        city_graph.add_edge_by_vertices(&"New York", &"Philadelphia");
        city_graph.add_edge_by_vertices(&"Philadelphia", &"Washington");

        for i in 0..city_graph.vertex_count() {
            println!(
                "{} -> {:?}",
                city_graph.vertex_at(i),
                city_graph.neighbors_for_index(i)
            );
        }

        let arena = Arena::new();
        let bfs_result = bfs(
            &arena,
            &"Boston",
            |x| x == &"Miami",
            |v| city_graph.neighbors_for_vertex(v),
        );

        if let Some(bfs_result) = bfs_result {
            let path = node_to_path(bfs_result);
            println!("Path from Boston to Miami:");
            println!("{:?}", path);
        } else {
            println!("No solution found using BFS!");
        }
        assert_eq!(
            node_to_path(bfs_result.unwrap()),
            vec!["Boston", "Detroit", "Washington", "Miami"]
        )
    }

    #[test]
    fn test_weighted_graph() {
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

        for i in 0..city_graph.vertex_count() {
            println!(
                "{} -> {:?}",
                city_graph.vertex_at(i),
                city_graph.neighbors_for_index_with_weights(i)
            );
        }
    }
}
