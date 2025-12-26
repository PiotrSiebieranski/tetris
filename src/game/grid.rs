use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

use super::colors::COLORS;

pub struct Grid {
    n_rows: usize,
    n_cols: usize,
    cellsize: usize,
    pub grid: [[usize; 10]; 20],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            n_rows: 20,
            n_cols: 10,
            cellsize: 30,
            grid: [[0; 10]; 20],
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                d.draw_rectangle(
                    (col * self.cellsize + 1) as i32,
                    (row * self.cellsize + 1) as i32,
                    (self.cellsize - 1) as i32,
                    (self.cellsize - 1) as i32,
                    COLORS[self.grid[row][col]],
                );
            }
        }
    }

    pub fn is_cell_outside(&self, row: usize, col: usize) -> bool {
        !(row < self.n_rows && col < self.n_cols)
    }
}
