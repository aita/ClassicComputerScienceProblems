use classic_computer_science_problems::generic_search::{astar, bfs, dfs, node_to_path, Arena};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Blocked,
    Start,
    Goal,
    Path,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MazeLocation {
    row: usize,
    column: usize,
}

#[derive(Debug, Clone)]
struct Maze {
    rows: usize,
    columns: usize,
    start: MazeLocation,
    goal: MazeLocation,
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    fn show(&self) {
        for row in 0..self.rows {
            for column in 0..self.columns {
                match self.grid[row][column] {
                    Cell::Empty => print!("  "),
                    Cell::Blocked => print!("🧱"),
                    Cell::Start => print!("🚦"),
                    Cell::Goal => print!("🏁"),
                    Cell::Path => print!("🚶"),
                }
            }
            println!();
        }
    }

    fn goal_test(&self, loc: &MazeLocation) -> bool {
        loc == &self.goal
    }

    fn successors(&self, loc: &MazeLocation) -> Vec<MazeLocation> {
        let mut locations = Vec::new();
        if loc.row + 1 < self.rows && self.grid[loc.row + 1][loc.column] != Cell::Blocked {
            locations.push(MazeLocation {
                row: loc.row + 1,
                column: loc.column,
            });
        }
        if loc.row > 0 && self.grid[loc.row - 1][loc.column] != Cell::Blocked {
            locations.push(MazeLocation {
                row: loc.row - 1,
                column: loc.column,
            });
        }
        if loc.column + 1 < self.columns && self.grid[loc.row][loc.column + 1] != Cell::Blocked {
            locations.push(MazeLocation {
                row: loc.row,
                column: loc.column + 1,
            });
        }
        if loc.column > 0 && self.grid[loc.row][loc.column - 1] != Cell::Blocked {
            locations.push(MazeLocation {
                row: loc.row,
                column: loc.column - 1,
            });
        }
        locations
    }

    fn marked(&self, path: Vec<MazeLocation>) -> Self {
        let mut maze = self.clone();
        for loc in path {
            maze.grid[loc.row][loc.column] = Cell::Path;
        }
        maze.grid[maze.start.row][maze.start.column] = Cell::Start;
        maze.grid[maze.goal.row][maze.goal.column] = Cell::Goal;
        maze
    }
}

#[derive(Default)]
struct MazeBuilder {
    rows: Option<usize>,
    columns: Option<usize>,
    sparseness: Option<f64>,
    start: Option<MazeLocation>,
    goal: Option<MazeLocation>,
}

impl MazeBuilder {
    fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    fn columns(mut self, columns: usize) -> Self {
        self.columns = Some(columns);
        self
    }

    fn sparseness(mut self, sparseness: f64) -> Self {
        self.sparseness = Some(sparseness);
        self
    }

    fn start(mut self, start: MazeLocation) -> Self {
        self.start = Some(start);
        self
    }

    fn goal(mut self, goal: MazeLocation) -> Self {
        self.goal = Some(goal);
        self
    }

    fn build(self) -> Maze {
        let rows = self.rows.unwrap_or(10);
        let columns = self.columns.unwrap_or(10);
        let sparseness = self.sparseness.unwrap_or(0.2);
        let start = self.start.unwrap_or(MazeLocation { row: 0, column: 0 });
        let goal = self.goal.unwrap_or(MazeLocation {
            row: rows - 1,
            column: columns - 1,
        });

        let mut grid = vec![vec![Cell::Empty; columns]; rows];

        // fill in the blocked cells with a random sparseness
        let mut rng = thread_rng();
        for row in 0..rows {
            for column in 0..columns {
                if rng.gen::<f64>() < sparseness {
                    grid[row][column] = Cell::Blocked;
                }
            }
        }

        grid[start.row][start.column] = Cell::Start;
        grid[goal.row][goal.column] = Cell::Goal;

        Maze {
            rows,
            columns,
            start,
            goal,
            grid,
        }
    }
}

#[allow(dead_code)]
fn euclidean_distance(goal: &MazeLocation) -> impl Fn(&MazeLocation) -> f64 + '_ {
    |loc| {
        let xdist = (loc.column as f64) - (goal.column as f64);
        let ydist = (loc.row as f64) - (goal.row as f64);
        (xdist * xdist + ydist * ydist).sqrt()
    }
}

fn manhattan_distance(goal: &MazeLocation) -> impl Fn(&MazeLocation) -> f64 + '_ {
    |loc| {
        let xdist = (loc.column as f64) - (goal.column as f64);
        let ydist = (loc.row as f64) - (goal.row as f64);
        xdist.abs() + ydist.abs()
    }
}

fn main() {
    let maze = MazeBuilder::default()
        .rows(10)
        .columns(10)
        .sparseness(0.2)
        .start(MazeLocation { row: 0, column: 0 })
        .goal(MazeLocation { row: 9, column: 9 })
        .build();

    println!("Maze:");
    maze.show();

    {
        println!("\nSolving maze using depth-first search:");
        let arena = Arena::new();
        let solution = dfs(
            &arena,
            &maze.start,
            |loc| maze.goal_test(loc),
            |loc| maze.successors(loc),
        );
        if let Some(node) = solution {
            let path = node_to_path(node);
            let maze = maze.marked(path);
            maze.show();
        } else {
            println!("No solution found using depth-first search!");
        }
    }

    {
        println!("\nSolving maze using breadth-first search:");
        let arena = Arena::new();
        let solution = bfs(
            &arena,
            &maze.start,
            |loc| maze.goal_test(loc),
            |loc| maze.successors(loc),
        );
        if let Some(node) = solution {
            let path = node_to_path(node);
            let maze = maze.marked(path);
            maze.show();
        } else {
            println!("No solution found using breadth-first search!");
        }
    }

    {
        println!("\nSolving maze using A* search:");
        let arena = Arena::new();
        let solution = astar(
            &arena,
            &maze.start,
            |loc| maze.goal_test(loc),
            |loc| maze.successors(loc),
            manhattan_distance(&maze.goal),
        );
        if let Some(node) = solution {
            let path = node_to_path(node);
            let maze = maze.marked(path);
            maze.show();
        } else {
            println!("No solution found using A* search!");
        }
    }
}
