use std::fmt;

use classic_computer_science_problems::genetic_algorithm::*;
use rand::seq::index::sample;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
struct SendMoreMoney2 {
    letters: Vec<char>,
}

impl fmt::Display for SendMoreMoney2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.letters.iter().position(|&c| c == 'S').unwrap() as isize;
        let e = self.letters.iter().position(|&c| c == 'E').unwrap() as isize;
        let n = self.letters.iter().position(|&c| c == 'N').unwrap() as isize;
        let d = self.letters.iter().position(|&c| c == 'D').unwrap() as isize;
        let m = self.letters.iter().position(|&c| c == 'M').unwrap() as isize;
        let o = self.letters.iter().position(|&c| c == 'O').unwrap() as isize;
        let r = self.letters.iter().position(|&c| c == 'R').unwrap() as isize;
        let y = self.letters.iter().position(|&c| c == 'Y').unwrap() as isize;
        let send = s * 1000 + e * 100 + n * 10 + d;
        let more = m * 1000 + o * 100 + r * 10 + e;
        let money = m * 10000 + o * 1000 + n * 100 + e * 10 + y;
        let diff = (money - (send + more)).abs();
        write!(
            f,
            "Send More Money {{ {} + {} = {}, Difference: {} }}",
            send, more, money, diff
        )
    }
}

impl Chromosome for SendMoreMoney2 {
    fn fitness(&self) -> f64 {
        let s = self.letters.iter().position(|&c| c == 'S').unwrap() as isize;
        let e = self.letters.iter().position(|&c| c == 'E').unwrap() as isize;
        let n = self.letters.iter().position(|&c| c == 'N').unwrap() as isize;
        let d = self.letters.iter().position(|&c| c == 'D').unwrap() as isize;
        let m = self.letters.iter().position(|&c| c == 'M').unwrap() as isize;
        let o = self.letters.iter().position(|&c| c == 'O').unwrap() as isize;
        let r = self.letters.iter().position(|&c| c == 'R').unwrap() as isize;
        let y = self.letters.iter().position(|&c| c == 'Y').unwrap() as isize;
        let send = s * 1000 + e * 100 + n * 10 + d;
        let more = m * 1000 + o * 100 + r * 10 + e;
        let money = m * 10000 + o * 1000 + n * 100 + e * 10 + y;
        let diff = (money - (send + more)).abs();
        1.0 / (diff as f64 + 1.0)
    }

    fn random_instance() -> Self {
        let mut rng = thread_rng();
        let mut letters = vec!['S', 'E', 'N', 'D', 'M', 'O', 'R', 'Y'];
        letters.shuffle(&mut rng);
        Self { letters }
    }

    fn crossover(&self, other: &Self) -> (Self, Self) {
        let mut child1 = self.clone();
        let mut child2 = other.clone();
        let indices = sample(&mut thread_rng(), self.letters.len(), 2);
        let (idx1, idx2) = (indices.index(0), indices.index(1));
        let (l1, l2) = (child1.letters[idx1], child2.letters[idx2]);
        let i = child1.letters.iter().position(|&c| c == l2).unwrap();
        child1.letters.swap(i, idx2);
        let j = child2.letters.iter().position(|&c| c == l1).unwrap();
        child2.letters.swap(j, idx1);
        (child1, child2)
    }

    fn mutate(&mut self) {
        let indices = sample(&mut thread_rng(), self.letters.len(), 2);
        let (idx1, idx2) = (indices.index(0), indices.index(1));
        self.letters.swap(idx1, idx2);
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let initial_population = (0..1000)
        .map(|_| SendMoreMoney2::random_instance())
        .collect::<Vec<_>>();
    let mut ga = GeneticAlgorithm::new(
        initial_population,
        1.0,
        1000,
        0.2,
        0.7,
        SelectionType::Roulette,
    );
    let result = ga.run();
    println!("{:}", result);
}
