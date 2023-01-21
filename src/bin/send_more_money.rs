use classic_computer_science_problems::csp::*;
use itertools::Itertools;
use std::collections::HashMap;

struct SendMoreMoneyConstraint {
    letters: Vec<char>,
}

impl Constraint<char, usize> for SendMoreMoneyConstraint {
    fn variables(&self) -> Vec<char> {
        self.letters.clone()
    }

    fn satisfied(&self, assignment: &HashMap<char, usize>) -> bool {
        if assignment.values().unique().count() < assignment.len() {
            return false;
        }

        if assignment.len() == self.letters.len() {
            let s = assignment[&'S'];
            let e = assignment[&'E'];
            let n = assignment[&'N'];
            let d = assignment[&'D'];
            let m = assignment[&'M'];
            let o = assignment[&'O'];
            let r = assignment[&'R'];
            let y = assignment[&'Y'];
            let send = s * 1000 + e * 100 + n * 10 + d;
            let more = m * 1000 + o * 100 + r * 10 + e;
            let money = m * 10000 + o * 1000 + n * 100 + e * 10 + y;
            send + more == money
        } else {
            true
        }
    }
}

fn main() {
    let letters = vec!['S', 'E', 'N', 'D', 'M', 'O', 'R', 'Y'];
    let mut possible_digits = HashMap::new();
    for letter in letters.iter() {
        possible_digits.insert(*letter, Vec::from_iter(0..10));
    }
    possible_digits.insert('M', vec![1]);
    let mut csp = CSP::new(letters.clone(), possible_digits);
    csp.add_constraint(SendMoreMoneyConstraint { letters });
    let solution = csp.backtracking_search();
    if let Some(solution) = solution {
        println!("Solution: {:?}", solution);
    } else {
        println!("No solution found!");
    }
}
