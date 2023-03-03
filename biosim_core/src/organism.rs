use crate::types;

#[derive(Debug)]
pub struct Gene {}

impl Gene {
    pub fn make_random() -> Gene {
        Gene {}
    }
}

pub type Genes = Vec<Gene>;

#[derive(Debug)]
pub struct Genome {
    pub genes: Genes,
}

impl Genome {
    pub fn make_random() -> Genome {
        Genome {
            genes: Genes::new(),
        }
    }
}

#[derive(Debug)]
pub struct Neuron {
    output: f64,
    driven: bool,
}

#[derive(Debug)]
pub struct NeuralNet {
    connections: Genes,
    neurons: Vec<Neuron>,
}

impl NeuralNet {
    pub fn new() -> Self {
        NeuralNet {
            connections: Genes::new(),
            neurons: Vec::<Neuron>::new(),
        }
    }
}

#[derive(Debug)]
pub struct Organizm {
    pub age: u32,
    pub alive: bool,
    pub birth_pos: types::Coord,
    pub pos: types::Coord,
    pub genome: Genome,
    pub net: NeuralNet,
    pub responsiveness: f64,
}

impl Organizm {
    pub fn new() -> Self {
        Organizm {
            age: 0,
            alive: true,
            birth_pos: types::Coord { x: 0, y: 0 },
            pos: types::Coord { x: 0, y: 0 },
            genome: Genome::make_random(),
            net: NeuralNet::new(),
            responsiveness: 1.0,
        }
    }

    pub fn feed_forward(&mut self, _step: u32) -> bool {
        true
    }

    pub fn execute_actions(&mut self, _level: bool) {}

    pub fn move_to(&mut self, dest: &types::Coord) {
        self.pos.x = dest.x;
        self.pos.y = dest.y;
    }
}
