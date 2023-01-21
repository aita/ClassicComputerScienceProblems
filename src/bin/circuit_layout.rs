use std::collections::HashMap;

use classic_computer_science_problems::csp::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CircuitLocation {
    row: usize,
    column: usize,
    rotated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Circuit(usize, usize);

fn generate_domain(circuit: &Circuit, rows: usize, columns: usize) -> Vec<CircuitLocation> {
    let mut domain = Vec::new();
    for row in 0..rows {
        for column in 0..columns {
            if row + circuit.0 <= rows && column + circuit.1 <= columns {
                domain.push(CircuitLocation {
                    row,
                    column,
                    rotated: false,
                });
            }
            // 90 degree rotation
            if circuit.0 != circuit.1 {
                if row + circuit.1 <= rows && column + circuit.0 <= columns {
                    domain.push(CircuitLocation {
                        row,
                        column,
                        rotated: true,
                    });
                }
            }
        }
    }
    domain
}

struct CircuitConstraint {
    circuit: Vec<Circuit>,
}

impl Constraint<Circuit, CircuitLocation> for CircuitConstraint {
    fn variables(&self) -> Vec<Circuit> {
        self.circuit.clone()
    }

    fn satisfied(&self, assignment: &HashMap<Circuit, CircuitLocation>) -> bool {
        let mut used = vec![vec![false; COLUMNS + 1]; ROWS + 1];
        for (circuit, location) in assignment {
            let (height, width) = if location.rotated {
                (circuit.1, circuit.0)
            } else {
                (circuit.0, circuit.1)
            };
            for row in location.row..location.row + height {
                for column in location.column..location.column + width {
                    if used[row][column] {
                        return false;
                    }
                    used[row][column] = true;
                }
            }
        }
        true
    }
}

const ROWS: usize = 9;
const COLUMNS: usize = 9;

fn main() {
    let circuits = vec![
        Circuit(4, 4),
        Circuit(3, 3),
        Circuit(1, 6),
        Circuit(2, 2),
        Circuit(2, 5),
    ];
    let mut domains = HashMap::new();
    for circuit in &circuits {
        domains.insert(circuit.clone(), generate_domain(circuit, ROWS, COLUMNS));
    }
    let mut csp = CSP::new(circuits.clone(), domains);
    csp.add_constraint(CircuitConstraint {
        circuit: circuits.clone(),
    });
    let solution = csp.backtracking_search();
    if let Some(solution) = solution {
        let mut grid = vec![vec![0; COLUMNS + 1]; ROWS + 1];
        for (circuit, location) in solution {
            let (height, width) = if location.rotated {
                (circuit.1, circuit.0)
            } else {
                (circuit.0, circuit.1)
            };
            for row in location.row..location.row + height {
                for column in location.column..location.column + width {
                    grid[row][column] = circuits.iter().position(|c| c == &circuit).unwrap() + 1;
                }
            }
        }
        for row in grid {
            for x in row {
                if x != 0 {
                    print!("{}", x);
                } else {
                    print!("x");
                }
            }
            println!();
        }
    } else {
        println!("No solution found!");
    }
}
