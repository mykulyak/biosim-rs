use crate::types;

#[derive(Debug)]
pub struct Grid {
    pub size_x: types::Dimension,
    pub size_y: types::Dimension,
    pub data: Vec<types::Coord>,
    pub barrier_locations: Vec<types::Coord>,
    pub barrier_centers: Vec<types::Coord>,
}

impl Grid {
    pub fn new(sx: u16, sy: u16) -> Grid {
        return Grid {
            size_x: sx,
            size_y: sy,
            data: Vec::with_capacity((sx * sy).into()),
            barrier_locations: Vec::new(),
            barrier_centers: Vec::new(),
        };
    }

    pub fn zero_fill(&mut self) {}

    pub fn create_barrier(&mut self, _barrier_type: u32) {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_fill() {
        let sx = 4;
        let sy = 4;
        let mut grid = Grid::new(sx, sy);

        for x in 0..sx {
            for y in 0..sy {
                grid.data.push(types::Coord { x, y });
            }
        }

        grid.zero_fill();

        for x in 0..sx {
            for y in 0..sy {
                let pt = &grid.data[(x * grid.size_x + y) as usize];
                assert_eq!(pt.x, x);
                assert_eq!(pt.y, y)
            }
        }
    }
}