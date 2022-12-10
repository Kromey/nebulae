use crate::color::Color;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;

const DEFAULT_SEED: u64 = 0xCAFEBABE;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stars {
    seed: u64,
    size: usize,
    cells: usize,
}

impl Stars {
    pub fn new(size: usize, cells: usize) -> Self {
        Self::seeded(size, cells, DEFAULT_SEED)
    }

    pub fn seeded(size: usize, cells: usize, seed: u64) -> Self {
        Self {
            seed,
            size,
            cells,
        }
    }

    pub fn generate(self) -> Vec<u16> {
        let mut sky = vec![Color::new(0.02, 0.02, 0.095, 1.0); self.size.pow(2)];

        let mut rng = Xoshiro256PlusPlus::seed_from_u64(self.seed);
        let num_cells = self.cells.pow(2);
        let cells: Vec<(f32, f32, f32)> = (0..num_cells)
            .map(|_| {
                let (x, y) = rng.gen::<(f32, f32)>();
                let radius = x.min(y).min(1.0 - x).min(1.0 - y) / 2.0;
                (x, y, radius)
            })
            .collect();

        let cell_size = self.size / self.cells;

        for (idx, cell) in cells.iter().enumerate() {
            if (cell.2 * cell_size as f32).round() <= f32::EPSILON {
                continue; // Don't waste time processing "invisible" stars
            }

            let (cell_x, cell_y) = self.get_cell_xy(idx);

            let offset_x = cell_x * cell_size;
            let offset_y = cell_y * cell_size;

            for x in 0..cell_size {
                for y in 0..cell_size {
                    let xf = x as f32 / cell_size as f32;
                    let yf = y as f32 / cell_size as f32;

                    let r = (1.0 - ((xf - cell.0).powi(2) + (yf - cell.1).powi(2)).sqrt() / cell.2).clamp(0.0, 1.0);
                    
                    if r > f32::EPSILON {
                        let idx = self.get_idx(x + offset_x, y + offset_y);
                        sky[idx] = sky[idx].blend(Color::new(r, r, r, 1.0));
                    }
                }
            }
        }

        sky.into_iter().flat_map(|color| color.to_array()).collect()
    }

    #[inline(always)]
    fn get_cell_xy(&self, cell: usize) -> (usize, usize) {
        (cell % self.cells, cell / self.cells)
    }

    fn get_idx(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }
}

