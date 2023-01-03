use classic_computer_science_problems::genetic_algorithm::*;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
struct SimpleEquation {
    x: i64,
    y: i64,
}

impl Chromosome for SimpleEquation {
    fn fitness(&self) -> f64 {
        // 6x - x^2 + 4y - y^2
        6.0 * self.x as f64 - self.x.pow(2) as f64 + 4.0 * self.y as f64 - self.y.pow(2) as f64
    }

    fn random_instance() -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(0..100),
            y: rng.gen_range(0..100),
        }
    }

    fn crossover(&self, other: &Self) -> (Self, Self) {
        (
            Self {
                x: self.x,
                y: other.y,
            },
            Self {
                x: other.x,
                y: self.y,
            },
        )
    }

    fn mutate(&mut self) {
        let mut rng = thread_rng();
        if rng.gen_bool(0.5) {
            if rng.gen_bool(0.5) {
                self.x += 1
            } else {
                self.x -= 1
            }
        } else {
            if rng.gen_bool(0.5) {
                self.y += 1
            } else {
                self.y -= 1
            }
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let initial_population = (0..20)
        .map(|_| SimpleEquation::random_instance())
        .collect::<Vec<_>>();

    let mut ga = GeneticAlgorithm::new(
        initial_population,
        13.0,
        100,
        0.1,
        0.7,
        SelectionType::Tournament,
    );
    let result = ga.run();
    println!(
        "X: {} Y: {} Fitness: {}",
        result.x,
        result.y,
        result.fitness()
    );
}
