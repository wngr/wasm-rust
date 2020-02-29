mod utils;

use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Conway's Game of Life
/// The universe of the Game of Life is an infinite two-dimensional orthogonal grid of square
/// cells, each of which is in one of two possible states, alive or dead, or "populated" or
/// "unpopulated". Every cell interacts with its eight neighbours, which are the cells that
/// are horizontally, vertically, or diagonally adjacent. At each step in time, the following
/// transitions occur:
///       (1) Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
///       (2) Any live cell with two or three live neighbours lives on to the next generation.
///       (3) Any live cell with more than three live neighbours dies, as if by overpopulation.
///       (4) Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
/// The initial pattern constitutes the seed of the system. The first generation is created by applying the
/// above rules simultaneously to every cell in the seed—births and deaths occur simultaneously, and the
/// discrete moment at which this happens is sometimes called a tick (in other words, in each
/// generation is a pure function of the preceding one). The rules continue to be applied
/// repeatedly to create further generations.
/// https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    generation: usize,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        let cells = (0..width * height)
            //            .map(|i| {
            //                if i % 3 == 0 || i % 7 == 0 {
            //                    Cell::Alive
            //                } else {
            //                    Cell::Dead
            //                }
            //            })
            .map(|_| {
                let x = rng.gen_range(0, 100);
                if x % 2 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            generation: 0,
            width,
            height,
            cells,
        }
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn tick(&mut self) -> usize {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_idx(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_cnt(row, col);
                let next_cell = match (cell, live_neighbours) {
                    // (1) Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // (2) Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // (3) Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // (4) Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
        self.generation += 1;
        self.generation
    }
    fn get_idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn live_neighbour_cnt(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for c in [self.width - 1, 0, 1].iter() {
            for r in [self.height - 1, 0, 1].iter() {
                // that's us
                if *c == 0 && *r == 0 {
                    continue;
                }
                let idx = self.get_idx((r + row) % self.height, (c + col) % self.width);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
