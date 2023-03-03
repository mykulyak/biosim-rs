use crate::{config, grid, population, signals, types};

#[derive(Debug)]
pub struct Simulator {
    pub mode: types::RunMode,
    pub config: config::Config,
    pub grid: grid::Grid,
    pub signals: signals::Signals,
    pub population: population::Population,
}

impl Simulator {
    pub fn new() -> Simulator {
        let config = config::Config::new();

        let sx = config.size_x;
        let sy = config.size_y;

        let grid = grid::Grid::new(sx, sy);
        let population = population::Population::new(config.population.try_into().unwrap());

        return Simulator {
            mode: types::RunMode::RUN,
            config: config,
            grid: grid,
            signals: signals::Signals::new(sx, sy),
            population: population,
        };
    }

    pub fn run(&mut self) {
        self.mode = types::RunMode::RUN;

        self.start_generation_0();

        for generation in 0..self.config.max_generations {
            println!("Starting generation {}", generation);

            if self.mode != types::RunMode::RUN {
                break;
            }

            let mut murder_count = 0;

            for step in 0..self.config.steps_per_generation {
                for organizm in &mut self.population.organizms {
                    organizm.age += 1;
                    let action_levels = organizm.feed_forward(step);
                    organizm.execute_actions(action_levels);
                }

                murder_count += self.population.death_queue_size();
                self.step_did_finish(step);
            }

            self.generation_did_finish(generation);
            let survivor_count = self.spawn_new_generation(generation, murder_count);
            if survivor_count > 0 && (generation % self.config.genome_analysis_stride == 0) {
                self.display_sample_genomes(self.config.display_sample_genomes);
            }

            if survivor_count == 0 {
                self.mode = types::RunMode::ABORT;
            }
        }

        self.display_sample_genomes(3);
    }

    fn start_generation_0(&mut self) {
        self.grid.zero_fill();
        self.grid.create_barrier(self.config.barrier_type);
        self.signals.zero_fill();
        self.population.initialize(self.config.population);
    }

    fn spawn_new_generation(&mut self, gen: u32, murdered: usize) -> usize {
        0
    }

    fn step_did_finish(&mut self, step: u32) {}

    fn generation_did_finish(&mut self, gen: u32) {}

    fn display_sample_genomes(&mut self, n: u32) {}

    pub fn done() {}
}
