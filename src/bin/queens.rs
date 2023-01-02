use std::collections::HashMap;

use classic_computer_science_problems::csp::*;

struct QueensConstraints {
    columns: Vec<usize>,
}

impl Constraint<usize, usize> for QueensConstraints {
    fn variables(&self) -> Vec<usize> {
        self.columns.clone()
    }

    fn satisfied(&self, assignment: &HashMap<usize, usize>) -> bool {
        for (q1c, q1r) in assignment {
            for q2c in (q1c + 1)..=self.columns.len() as usize {
                if let Some(q2r) = assignment.get(&q2c) {
                    if q1r == q2r {
                        return false;
                    }
                    if (*q1r as isize - *q2r as isize).abs() == (*q1c as isize - q2c as isize).abs()
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn main() {
    let columns = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut rows = HashMap::new();
    for column in &columns {
        rows.insert(column.clone(), columns.clone());
    }
    let mut csp = CSP::new(columns.clone(), rows);
    csp.add_constraint(QueensConstraints { columns });
    let solution = csp.backtracking_search();
    if let Some(solution) = solution {
        println!("Solution: {:?}", solution);
    } else {
        println!("No solution found!");
    }
}
