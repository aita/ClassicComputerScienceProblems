use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub trait Constraint<V, D> {
    fn variables(&self) -> Vec<V>;
    fn satisfied(&self, arguments: &HashMap<V, D>) -> bool;
}

pub struct CSP<V, D, C>
where
    V: Eq + Hash + Clone,
    D: Eq + Hash + Clone,
    C: Constraint<V, D>,
{
    variables: Vec<V>,
    domains: HashMap<V, Vec<D>>,
    constraints: HashMap<V, Vec<Rc<C>>>,
}

impl<V, D, C> CSP<V, D, C>
where
    V: Eq + Hash + Clone,
    D: Eq + Hash + Clone,
    C: Constraint<V, D>,
{
    pub fn new(variables: Vec<V>, domains: HashMap<V, Vec<D>>) -> Self {
        let mut constraints = HashMap::new();
        for variable in &variables {
            constraints.insert(variable.clone(), Vec::new());
            if !domains.contains_key(variable) {
                panic!("Every variable should have a domain assigned to it.");
            }
        }
        Self {
            variables,
            domains,
            constraints,
        }
    }

    pub fn add_constraint(&mut self, constraint: C) {
        let constraint = Rc::from(constraint);
        for variable in constraint.variables() {
            if !self.variables.contains(&variable) {
                panic!("Variable in constraint not in CSP");
            }
            self.constraints
                .get_mut(&variable)
                .unwrap()
                .push(constraint.clone());
        }
    }

    pub fn consistent(&self, variable: &V, assignment: &HashMap<V, D>) -> bool {
        self.constraints[variable]
            .iter()
            .all(|c| c.satisfied(assignment))
    }

    pub fn backtracking_search(&self) -> Option<HashMap<V, D>> {
        self.backtrack(HashMap::new())
    }

    fn backtrack(&self, assignment: HashMap<V, D>) -> Option<HashMap<V, D>> {
        if assignment.len() == self.variables.len() {
            return Some(assignment);
        }

        let mut unassigned = self
            .variables
            .iter()
            .filter(|v| !assignment.contains_key(v));

        let first = unassigned.next().unwrap();
        for value in &self.domains[first] {
            let mut local_assignment = assignment.clone();
            local_assignment.insert(first.clone(), value.clone());
            if self.consistent(first, &local_assignment) {
                let result = self.backtrack(local_assignment);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }
}
