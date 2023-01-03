use classic_computer_science_problems::generic_search::{bfs, node_to_path, Arena};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MCState<const MAX_NUM: usize = 3> {
    wm: usize,
    wc: usize,
    em: usize,
    ec: usize,
    boat: bool,
}

impl<const MAX_NUM: usize> MCState<MAX_NUM> {
    fn new(missionaries: usize, cannibals: usize, boat: bool) -> Self {
        Self {
            wm: missionaries,
            wc: cannibals,
            em: MAX_NUM - missionaries,
            ec: MAX_NUM - cannibals,
            boat: boat,
        }
    }

    fn show(&self) {
        println!(
            "On the west bank: {} missionaries, {} cannibals.",
            self.wm, self.wc
        );
        println!(
            "On the east bank: {} missionaries, {} cannibals.",
            self.em, self.ec
        );
        println!(
            "The boat is on the {} bank.",
            if self.boat { "west" } else { "east" }
        );
    }

    fn goal_test(&self) -> bool {
        self.is_legal() && self.em == MAX_NUM && self.ec == MAX_NUM
    }

    fn is_legal(&self) -> bool {
        if self.wm < self.wc && self.wm > 0 {
            false
        } else if self.em < self.ec && self.em > 0 {
            false
        } else {
            true
        }
    }

    fn successors(&self) -> Vec<Self> {
        let mut succs = Vec::new();
        if self.boat {
            if self.wm > 1 {
                succs.push(MCState::new(self.wm - 2, self.wc, !self.boat));
            }
            if self.wm > 0 {
                succs.push(MCState::new(self.wm - 1, self.wc, !self.boat));
            }
            if self.wc > 1 {
                succs.push(MCState::new(self.wm, self.wc - 2, !self.boat));
            }
            if self.wc > 0 {
                succs.push(MCState::new(self.wm, self.wc - 1, !self.boat));
            }
            if self.wm > 0 && self.wc > 0 {
                succs.push(MCState::new(self.wm - 1, self.wc - 1, !self.boat));
            }
        } else {
            if self.em > 1 {
                succs.push(MCState::new(self.wm + 2, self.wc, !self.boat));
            }
            if self.em > 0 {
                succs.push(MCState::new(self.wm + 1, self.wc, !self.boat));
            }
            if self.ec > 1 {
                succs.push(MCState::new(self.wm, self.wc + 2, !self.boat));
            }
            if self.ec > 0 {
                succs.push(MCState::new(self.wm, self.wc + 1, !self.boat));
            }
            if self.em > 0 && self.ec > 0 {
                succs.push(MCState::new(self.wm + 1, self.wc + 1, !self.boat));
            }
        }
        succs.into_iter().filter(|x| x.is_legal()).collect()
    }
}

fn display_solution(solution: Vec<MCState>) {
    if solution.is_empty() {
        return;
    }
    let mut old_state = &solution[0];
    old_state.show();
    for state in solution.iter().skip(1) {
        if state.boat {
            println!(
                "{} missionaries and {} cannibals moved from the east bank to the west bank.",
                old_state.em - state.em,
                old_state.ec - state.ec
            );
        } else {
            println!(
                "{} missionaries and {} cannibals moved from the west bank to the east bank.",
                old_state.wm - state.wm,
                old_state.wc - state.wc
            );
        }
        state.show();
        old_state = state;
    }
}

fn main() {
    let start = MCState::new(3, 3, true);
    let goal = |x: &MCState| x.goal_test();
    let successors = |x: &MCState| x.successors();
    let arena = Arena::new();
    let solution = bfs(&arena, &start, goal, successors);
    if let Some(node) = solution {
        let path = node_to_path(node);
        display_solution(path);
    } else {
        println!("No solution found!");
    }
}
