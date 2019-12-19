use crate::cell::Cell;
use crate::fitness::FitnessEvaluator;

use rand::prelude::*;
use rand::seq::SliceRandom;

use std::collections::HashMap;
use std::cmp::Ordering::Less;
use std::cmp::max;


pub struct Parameters<T> where T:FitnessEvaluator {
    pub population_size: usize,
    pub genetic_code_length: usize,

    pub keep_threshold: f64,

    pub mutation_chance_percent: f64,

    pub fitness_evaluator: T,
}


pub struct Simulation<T> where T:FitnessEvaluator {
    params: Parameters<T>,
    current_id: u64
}

impl <T> Simulation<T> where T: FitnessEvaluator {
    pub fn new(p: Parameters<T>) -> Simulation<T> {
        return Simulation{
            params: p,
            current_id: 0
        };
    }

    fn generate_population(&mut self) -> HashMap<u64, Cell> {
        let mut population: HashMap<u64, Cell> = HashMap::new();
        population.reserve(self.params.population_size);

        for _i in 0..self.params.population_size {
            let random_bytes: Vec<u8> = (0..self.params.genetic_code_length).map(|_| { rand::random::<u8>() }).collect();
            population.insert(self.current_id, Cell::new(self.current_id, random_bytes));
            self.current_id += 1;
        }

        return population;
    }

    fn evaluate_fitness(&self, population: &mut HashMap<u64, Cell>) {
        for (_cell_id, cell) in population.iter_mut() {
            cell.score = self.params.fitness_evaluator.evaluate(&cell.genetic_code);
        }
    }

    fn select_population(&self, population: &mut HashMap<u64, Cell>) {
        let mut selected_individual_ids: Vec<u64> = Vec::new();
        for cell_id in population.keys() {
            selected_individual_ids.push(*cell_id);
        }
        selected_individual_ids.sort_by(|a, b| population[a].score.partial_cmp(&population[b].score).unwrap_or(Less));

        let id_of_best_cell = selected_individual_ids.get(selected_individual_ids.len() - 1).unwrap();
        println!("Best score: {}", population.get(id_of_best_cell).unwrap().score);

        let amount_to_trim = ((selected_individual_ids.len() as f64) * (1.0 - self.params.keep_threshold)) as usize;
        let amount_to_trim = max(amount_to_trim, 1);

        for id_to_remove in selected_individual_ids[..amount_to_trim].iter() {
            population.remove(id_to_remove).unwrap();
        }
    }

    fn breed(&mut self, population: &mut HashMap<u64, Cell>) {
        let mut parents_ids: Vec<u64> = Vec::new();
        for cell_id in population.keys() {
            parents_ids.push(*cell_id);
        }

        for _i in 0..(self.params.population_size - population.len()) {
            let parents: Vec<u64> = parents_ids.choose_multiple(&mut rand::thread_rng(), 2).cloned().collect();
            assert_eq!(parents.len(), 2);
            let new_genetic_code = population.get(&parents[0]).unwrap().breed(population.get(&parents[1]).unwrap());
            population.insert(self.current_id, Cell::new(self.current_id, new_genetic_code));
            self.current_id += 1;
        }
    }

    fn do_mutations(&self, population: &mut HashMap<u64, Cell>) {
        let mut rng = rand::thread_rng();
        for (_key, cell) in population.iter_mut() {
            let throw: f64 = rng.gen();
            if throw > self.params.mutation_chance_percent {
                continue;
            }

            // Mutate the cell.
            for i in 0..self.params.genetic_code_length {
                let mut_throw: f64 = rng.gen();
                if mut_throw > 0.99 {
                    let mut mutation_range: f64 = 50.0;
                    let mut_throw: f64 = rng.gen();
                    if mut_throw > 0.999 {
                        mutation_range = 250.0;
                    }

                    let range_lower_bound: f64 = (cell.genetic_code[i] - ((mutation_range as i32) / 2) as u8) as f64;

                    let multiplier: f64 = rng.gen();
                    cell.genetic_code[i] = (range_lower_bound + multiplier * mutation_range) as u8;
                }
            }
        }
    }

    pub fn run(&mut self) {
        let mut population = self.generate_population();

        for _i in 0..30000 {
            self.evaluate_fitness(&mut population);
            self.select_population(&mut population);
            self.breed(&mut population);
            self.do_mutations(&mut population);
        }
    }
}
