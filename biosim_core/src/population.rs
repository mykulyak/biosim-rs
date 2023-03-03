use crate::{organism, types};

#[derive(Debug)]
struct Move {
    index: u16,
    to: types::Coord,
}

#[derive(Debug)]
pub struct Population {
    pub organizms: Vec<organism::Organizm>,
    death_queue: Vec<u16>,
    move_queue: Vec<Move>,
}

impl Population {
    pub fn new(count: usize) -> Self {
        let mut organizms = Vec::<organism::Organizm>::with_capacity(count);
        for _i in 0..count {
            organizms.push(organism::Organizm::new());
        }

        Self {
            organizms,
            death_queue: Vec::<u16>::new(),
            move_queue: Vec::<Move>::new(),
        }
    }

    pub fn initialize(&mut self, _count: u32) {}

    pub fn death_queue_size(&self) -> usize {
        return self.death_queue.len();
    }

    pub fn do_move(&mut self) {
        for m in self.move_queue.iter() {
            self.organizms[m.index as usize].move_to(&m.to);
        }
    }
}
