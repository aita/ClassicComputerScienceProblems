use classic_computer_science_problems::csp::*;
use std::collections::HashMap;

struct MapColoringConstraint<'a>(&'a str, &'a str);

impl<'a> Constraint<&'a str, &'a str> for MapColoringConstraint<'a> {
    fn variables(&self) -> Vec<&'a str> {
        vec![self.0, self.1]
    }

    fn satisfied(&self, assignment: &HashMap<&str, &str>) -> bool {
        if !assignment.contains_key(self.0) || !assignment.contains_key(self.1) {
            true
        } else {
            assignment[self.0] != assignment[self.1]
        }
    }
}

fn main() {
    let variables = vec![
        "Western Australia",
        "Northern Territory",
        "South Australia",
        "Queensland",
        "New South Wales",
        "Victoria",
        "Tasmania",
    ];
    let domains: HashMap<&str, Vec<&str>> = variables
        .iter()
        .map(|variable| (*variable, vec!["red", "green", "blue"]))
        .collect();
    let mut csp = CSP::new(variables, domains);
    csp.add_constraint(MapColoringConstraint(
        "Western Australia",
        "Northern Territory",
    ));
    csp.add_constraint(MapColoringConstraint(
        "Western Australia",
        "South Australia",
    ));
    csp.add_constraint(MapColoringConstraint(
        "South Australia",
        "Northern Territory",
    ));
    csp.add_constraint(MapColoringConstraint("Queensland", "Northern Territory"));
    csp.add_constraint(MapColoringConstraint("Queensland", "South Australia"));
    csp.add_constraint(MapColoringConstraint("Queensland", "New South Wales"));
    csp.add_constraint(MapColoringConstraint("New South Wales", "South Australia"));
    csp.add_constraint(MapColoringConstraint("Victoria", "South Australia"));
    csp.add_constraint(MapColoringConstraint("Victoria", "New South Wales"));
    csp.add_constraint(MapColoringConstraint("Victoria", "Tasmania"));
    let solution = csp.backtracking_search();
    if let Some(solution) = solution {
        println!("Solution: {:?}", solution);
    } else {
        println!("No solution found!");
    }
}
