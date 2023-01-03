use approx::relative_eq;
use bumpalo::Bump;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;

pub struct Arena<T> {
    bump: Bump,
    phantom: PhantomData<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            bump: Bump::new(),
            phantom: PhantomData,
        }
    }

    pub fn add_node<'bump>(&'bump self, node: Node<'bump, T>) -> &'bump Node<T> {
        let node = self.bump.alloc(node);
        node
    }
}

#[derive(Debug)]
pub struct Node<'bump, T> {
    pub state: T,
    pub parent: Option<&'bump Node<'bump, T>>,
    pub cost: f64,
    pub heuristic: f64,
}

impl<'bump, T> Node<'bump, T> {
    pub fn new(state: T, parent: Option<&'bump Node<'bump, T>>) -> Self {
        Self {
            state,
            parent: parent,
            cost: 0.0,
            heuristic: 0.0,
        }
    }

    pub fn with_heuristic(
        state: T,
        parent: Option<&'bump Node<'bump, T>>,
        cost: f64,
        heuristic: f64,
    ) -> Self {
        Self {
            state,
            parent: parent,
            cost,
            heuristic,
        }
    }
}

impl<'bump, T> PartialEq for Node<'bump, T> {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.cost + self.heuristic, other.cost + other.heuristic)
    }
}

impl<'bump, T> Eq for Node<'bump, T> {}

impl<'bump, T> Ord for Node<'bump, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_priority = self.cost + self.heuristic;
        let other_priority = other.cost + other.heuristic;
        other_priority.partial_cmp(&self_priority).unwrap()
    }
}

impl<'bump, T> PartialOrd for Node<'bump, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dfs<'bump, T, G, S>(
    arena: &'bump Arena<T>,
    initial: &T,
    goal_test: G,
    successors: S,
) -> Option<&'bump Node<'bump, T>>
where
    T: Eq + Hash + Clone,
    G: Fn(&T) -> bool,
    S: Fn(&T) -> Vec<T>,
{
    let mut frontier = Vec::new();
    frontier.push(arena.add_node(Node::new(initial.clone(), None)));

    let mut explored = HashSet::new();
    explored.insert(initial.clone());

    while let Some(node) = frontier.pop() {
        if goal_test(&node.state) {
            return Some(node);
        }
        for child in successors(&node.state) {
            if !explored.contains(&child) {
                explored.insert(child.clone());
                frontier.push(arena.add_node(Node::new(child.clone(), Some(node))));
            }
        }
    }

    None
}

pub fn node_to_path<T: Clone>(node: &Node<T>) -> Vec<T> {
    let mut path = vec![node.state.clone()];
    let mut current = node;
    while let Some(parent) = current.parent {
        current = parent;
        path.push(current.state.clone());
    }
    path.reverse();
    path
}

pub fn bfs<'bump, T, G, S>(
    arena: &'bump Arena<T>,
    initial: &T,
    goal_test: G,
    successors: S,
) -> Option<&'bump Node<'bump, T>>
where
    T: Eq + Hash + Clone,
    G: Fn(&T) -> bool,
    S: Fn(&T) -> Vec<T>,
{
    let mut frontier = VecDeque::new();
    frontier.push_back(arena.add_node(Node::new(initial.clone(), None)));

    let mut explored = HashSet::new();
    explored.insert(initial.clone());

    while let Some(node) = frontier.pop_front() {
        if goal_test(&node.state) {
            return Some(node);
        }
        for child in successors(&node.state) {
            if !explored.contains(&child) {
                explored.insert(child.clone());
                frontier.push_back(arena.add_node(Node::new(child.clone(), Some(node))));
            }
        }
    }

    None
}

pub fn astar<'bump, T, G, S, H>(
    arena: &'bump Arena<T>,
    initial: &T,
    goal_test: G,
    successors: S,
    heuristic: H,
) -> Option<&'bump Node<'bump, T>>
where
    T: Eq + Hash + Clone,
    G: Fn(&T) -> bool,
    S: Fn(&T) -> Vec<T>,
    H: Fn(&T) -> f64,
{
    let mut frontier = BinaryHeap::new();
    frontier.push(arena.add_node(Node::with_heuristic(
        initial.clone(),
        None,
        0.0,
        heuristic(initial),
    )));

    let mut explored = HashMap::new();
    explored.insert(initial.clone(), 0.0);

    while let Some(node) = frontier.pop() {
        if goal_test(&node.state) {
            return Some(node);
        }
        for child in successors(&node.state) {
            let new_cost = node.cost + 1.0;
            if explored.get(&child).is_none() || explored[&child] > new_cost {
                explored.insert(child.clone(), new_cost);
                let node =
                    Node::with_heuristic(child.clone(), Some(node), new_cost, heuristic(&child));
                frontier.push(arena.add_node(node));
            }
        }
    }

    None
}
