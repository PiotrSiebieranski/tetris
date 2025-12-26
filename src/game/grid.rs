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

    pub fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col] == 0
    }

    pub fn clear_full_rows(&mut self) -> usize {
        let mut compleated = 0usize;
        for row in (0..self.n_rows).rev() {
            if self.is_row_full(row) {
                self.clear_row(row);
                compleated += 1;
            } else if compleated > 0 {
                self.move_down(row, compleated);
            }
        }

        compleated
    }

    fn is_row_full(&self, row: usize) -> bool {
        self.grid[row].iter().all(|&cell| cell != 0)
    }

    fn clear_row(&mut self, row: usize) {
        self.grid[row].fill(0);
    }

    fn move_down(&mut self, row: usize, n_rows: usize) {
        for r in (0..=row).rev() {
            let target = r + n_rows;
            if target < self.n_rows {
                for c in 0..self.n_cols {
                    self.grid[target][c] = self.grid[r][c];
                    self.grid[r][c] = 0;
                }
            }
        }
    }
}
