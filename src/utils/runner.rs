use std::fmt::{Display, Formatter};
use ndarray::Array2;
use float_eq::float_eq;
use rand::seq::SliceRandom;
use crate::global_params::CgpParameters as g_params;
#[cfg(feature = "vanilla")]
use crate::vanilla_cgp::chromosome::Chromosome;
#[cfg(feature = "dag")]
use crate::dag::chromosome_dag::Chromosome;
#[cfg(feature = "reorder")]
use crate::reorder::chromosome_reorder::Chromosome;
#[cfg(feature = "ereorder")]
use crate::reorder::chromosome_reorder_equidistant::Chromosome;


pub struct Runner {
    params: g_params,
    data: Array2<bool>,
    label: Array2<bool>,
    chromosomes: Vec<Chromosome>,
    best_fitness: f32,
    fitness_vals: Vec<f32>,
    parent_id: usize,
    mutation_type: usize,
    mutation_prob: f32,
}

impl Display for Runner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parent: {}", self.chromosomes[self.parent_id])?;
        writeln!(f, "Fitness: {}", self.best_fitness)
    }
}

impl Runner {
    pub fn new(params: g_params,
               data: Array2<bool>,
               label: Array2<bool>,
               mut_type: usize,
               mut_prob: f32) -> Self {
        let mut chromosomes: Vec<Chromosome> = Vec::with_capacity(params.mu + params.lambda);
        let mut fitness_vals: Vec<f32> = Vec::with_capacity(params.mu + params.lambda);

        for _ in 0..(params.mu + params.lambda) {
            let mut chromosome = Chromosome::new(params.clone());
            let fitness = chromosome.evaluate(&data, &label);
            fitness_vals.push(fitness);

            chromosomes.push(chromosome);
        }

        let best_fitness = get_min(&fitness_vals);
        let parent_id = get_argmin(&fitness_vals);

        Self {
            params,
            data,
            label,
            chromosomes,
            best_fitness,
            fitness_vals,
            parent_id,
            mutation_type: mut_type,
            mutation_prob: mut_prob,
        }
    }

    pub fn learn_step(&mut self) {
        self.mutate_chromosomes();

        self.eval_chromosomes();

        self.new_parent_by_neutral_search();
    }

    fn new_parent_by_neutral_search(&mut self) {
        let mut min_keys: Vec<usize> = Vec::with_capacity(self.params.mu + self.params.lambda);

        get_argmins_of_value(&self.fitness_vals, &mut min_keys, self.best_fitness);

        if min_keys.len() == 1 {
            self.parent_id = min_keys[0];
        } else {
            if min_keys.contains(&self.parent_id) {
                let index = min_keys.iter().position(|x| *x == self.parent_id).unwrap();
                min_keys.remove(index);
            }
            self.parent_id = *min_keys.choose(&mut rand::thread_rng()).unwrap();
        }
    }

    fn mutate_chromosomes(&mut self) {
        // mutate new chromosomes; do not mutate parent
        for i in 0..(self.params.mu + self.params.lambda) {
            if i == self.parent_id {
                continue;
            }

            self.chromosomes[i] = self.chromosomes[self.parent_id].clone();

            match self.mutation_type {
                0 => { self.chromosomes[i].mutate_single(); }
                1 => { self.chromosomes[i].mutate_prob(self.mutation_prob); }

                _ => { panic!("mutatio ntype not def") }
            }
        }
    }

    fn eval_chromosomes(&mut self) {
        for i in 0..(self.params.mu + self.params.lambda) {
            if i != self.parent_id {
                let fitness = self.chromosomes[i].evaluate(&self.data, &self.label);
                self.fitness_vals[i] = fitness;

                // TODO CHECK HERE
                self.chromosomes[i].reorder();
                let fitness = self.chromosomes[i].evaluate(&self.data, &self.label);
                assert_eq!(self.fitness_vals[i], fitness);
            }
        }

        let best_fitness = get_min(&self.fitness_vals);

        self.best_fitness = best_fitness;
    }

    pub fn get_best_fitness(&self) -> f32 {
        return self.best_fitness;
    }

    pub fn get_parent(&self) -> Chromosome {
        return self.chromosomes[self.parent_id].clone();
    }
}


fn get_argmin(nets: &Vec<f32>) -> usize {
    nets.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)
        .unwrap()
}

fn get_min(nets: &Vec<f32>) -> f32 {
    *nets.into_iter()
        .min_by(|a, b| a.partial_cmp(b)
            .unwrap())
        .unwrap()
}

fn get_argmins_of_value(vecs: &Vec<f32>, res: &mut Vec<usize>, comp_value: f32) {
    vecs.iter()
        .enumerate()
        .for_each(|(i, v)| {
            if float_eq!(*v, comp_value, abs <= 0.000_1) {
                res.push(i);
            }
        });
}