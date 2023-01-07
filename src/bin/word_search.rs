use std::collections::{HashMap, HashSet};

use classic_computer_science_problems::csp::*;
use rand::prelude::*;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GridLocation {
    row: usize,
    column: usize,
}

fn generate_grid(rows: usize, columns: usize) -> Grid {
    let mut rng = rand::thread_rng();
    (0..rows)
        .map(|_| {
            (0..columns)
                .map(|_| ('A'..='Z').choose(&mut rng).unwrap())
                .collect()
        })
        .collect()
}

fn display_grid(grid: &Grid) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn generate_domain(length: usize, width: usize, height: usize) -> Vec<Vec<GridLocation>> {
    let mut domain = Vec::new();
    for row_index in 0..height {
        for column_index in 0..width {
            let columns = || column_index..column_index + length + 1;
            let rows = || row_index..row_index + length + 1;
            if column_index + length <= width {
                domain.push(
                    columns()
                        .map(|c| GridLocation {
                            row: row_index,
                            column: c,
                        })
                        .collect(),
                );
                if row_index + length <= height {
                    domain.push(
                        rows()
                            .map(|r| GridLocation {
                                row: r,
                                column: column_index + r - row_index,
                            })
                            .collect(),
                    )
                }
            }
            if row_index + length <= height {
                domain.push(
                    rows()
                        .map(|r| GridLocation {
                            row: r,
                            column: column_index,
                        })
                        .collect(),
                );
                if column_index >= length {
                    domain.push(
                        rows()
                            .map(|r| GridLocation {
                                row: r,
                                column: column_index + row_index - r,
                            })
                            .collect(),
                    )
                }
            }
        }
    }
    domain
}

struct WordSearchConstraint<'a> {
    words: Vec<&'a str>,
}

impl<'a> Constraint<&'a str, Vec<GridLocation>> for WordSearchConstraint<'a> {
    fn variables(&self) -> Vec<&'a str> {
        self.words.clone()
    }

    fn satisfied(&self, assignment: &HashMap<&'a str, Vec<GridLocation>>) -> bool {
        let all_locations = assignment.values().flatten().collect::<Vec<_>>();
        let location_set = all_locations.iter().collect::<HashSet<_>>();
        location_set.len() == all_locations.len()
    }
}

fn main() {
    const ROWS: usize = 9;
    const COLUMNS: usize = 9;
    let mut rng = rand::thread_rng();

    let words = vec!["MATTHEW", "JOE", "MARY", "SARAH", "SALLY"];
    let mut locations = HashMap::new();
    for word in words.iter().cloned() {
        locations.insert(word, generate_domain(word.len(), ROWS, COLUMNS));
    }
    let mut csp = CSP::new(words.clone(), locations);
    csp.add_constraint(WordSearchConstraint { words });
    let solution = csp.backtracking_search();

    let mut grid = generate_grid(ROWS, COLUMNS);
    if let Some(solution) = solution {
        for (word, mut grid_locations) in solution {
            if rng.gen_bool(0.5) {
                grid_locations.reverse();
            }
            for (index, letter) in word.chars().enumerate() {
                let GridLocation { row, column } = grid_locations[index];
                grid[row][column] = letter;
            }
        }
        display_grid(&grid);
    } else {
        println!("No solution found!");
    }
}
