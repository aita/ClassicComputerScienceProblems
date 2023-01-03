use std::{cell::RefCell, rc::Rc};

use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::{seq::SliceRandom, thread_rng, Rng};

/// 染色体
pub trait Chromosome: Sized + Clone {
    fn fitness(&self) -> f64;
    fn random_instance() -> Self;
    fn crossover(&self, other: &Self) -> (Self, Self);
    fn mutate(&mut self);
}

pub enum SelectionType {
    Roulette,
    Tournament,
}

pub struct GeneticAlgorithm<C>
where
    C: Chromosome,
{
    population: Vec<Rc<RefCell<C>>>,
    threshold: f64,
    max_generations: u32,
    mutation_chance: f64,
    crossover_chance: f64,
    selection_type: SelectionType,
}

impl<C> GeneticAlgorithm<C>
where
    C: Chromosome,
{
    pub fn new(
        initial_population: Vec<C>,
        threshold: f64,
        max_generations: u32,
        mutation_chance: f64,
        crossover_chance: f64,
        selection_type: SelectionType,
    ) -> Self {
        let population = initial_population
            .into_iter()
            .map(|c| Rc::new(RefCell::new(c)))
            .collect();
        Self {
            population,
            threshold,
            max_generations,
            mutation_chance,
            crossover_chance,
            selection_type,
        }
    }

    /// 確率分布のルーレットで 2 つの親を選ぶ。
    /// 注記：負の適応度ではうまくいかない
    fn pick_roulette(&self, wheel: &[f64]) -> (Rc<RefCell<C>>, Rc<RefCell<C>>) {
        let mut rng = thread_rng();
        let choices = self.population.iter().zip(wheel.iter()).collect::<Vec<_>>();
        let picked = choices
            .choose_multiple_weighted(&mut rng, 2, |x| *x.1)
            .unwrap();
        picked.map(|x| x.0.clone()).next_tuple().unwrap()
    }

    /// num 個の無作為抽出した個体から最良の 2 つを選ぶ
    fn pick_tournament(&self, num_participants: usize) -> (Rc<RefCell<C>>, Rc<RefCell<C>>) {
        let mut rng = thread_rng();
        let picked = self.population.choose_multiple(&mut rng, num_participants);
        let sorted = picked.sorted_by_key(|x| OrderedFloat(-x.borrow().fitness()));
        sorted.cloned().next_tuple().unwrap()
    }

    fn reproduce_and_replace(&mut self) {
        let mut rng = thread_rng();
        let mut new_population = Vec::new();
        while new_population.len() < self.population.len() {
            // 両親を選ぶ
            let (parent1, parent2) = match self.selection_type {
                SelectionType::Roulette => {
                    let wheel = self
                        .population
                        .iter()
                        .map(|x| x.borrow().fitness())
                        .collect::<Vec<_>>();
                    self.pick_roulette(&wheel)
                }
                SelectionType::Tournament => self.pick_tournament(self.population.len() / 2),
            };
            // 両親を確立的に交叉させる
            if rng.gen_bool(self.crossover_chance) {
                let (child1, child2) = parent1.borrow().crossover(&parent2.borrow());
                new_population.push(Rc::new(RefCell::new(child1)));
                new_population.push(Rc::new(RefCell::new(child2)));
            } else {
                new_population.push(parent1.clone());
                new_population.push(parent2.clone());
            }
        }
        // もし奇数だったら、1つ余分になるので、mutそれを削除する
        if new_population.len() < self.population.len() {
            new_population.pop();
        }
        self.population = new_population;
    }

    fn mutate(&mut self) {
        let mut rng = thread_rng();
        for individual in self.population.iter() {
            if rng.gen_bool(self.mutation_chance) {
                individual.as_ref().borrow_mut().mutate();
            }
        }
    }

    pub fn run(&mut self) -> C {
        let mut best = self
            .population
            .iter()
            .max_by_key(|x| OrderedFloat(x.borrow().fitness()))
            .unwrap()
            .clone();
        for generation in 0..self.max_generations {
            if best.borrow().fitness() >= self.threshold {
                break;
            }
            tracing::info!(
                "Generation {} Best {} Avg {}",
                generation,
                best.borrow().fitness(),
                self.population
                    .iter()
                    .map(|x| x.borrow().fitness())
                    .sum::<f64>()
                    / self.population.len() as f64
            );
            self.reproduce_and_replace();
            self.mutate();
            let highest = self
                .population
                .iter()
                .max_by_key(|x| OrderedFloat(x.borrow().fitness()))
                .unwrap();
            if highest.borrow().fitness() > best.borrow().fitness() {
                best = highest.clone();
            }
        }
        let cloned = best.borrow().clone();
        cloned
    }
}
