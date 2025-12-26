use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

use super::colors::COLORS;

pub struct Block {
    pub kind: BlockKind,
    cells: [&'static [(usize, usize)]; 4],
    cellsize: usize,
    rotation_state: usize,
    row_offset: isize,
    coll_offset: isize,
}

impl Block {
    pub fn new(kind: BlockKind) -> Self {
        let mut ret = Self {
            cellsize: 30,
            rotation_state: 0,
            cells: kind.cells(),
            kind,
            row_offset: 0,
            coll_offset: 0,
        };

        match kind {
            BlockKind::O => ret.move_by(0, 4),
            BlockKind::I => ret.move_by(0, 3),
            _ => ret.move_by(0, 3),
        }
        ret
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let tiles = self.get_cell_position();
        for tile in tiles {
            d.draw_rectangle(
                (tile.0 * self.cellsize + 1) as i32,
                (tile.1 * self.cellsize + 1) as i32,
                (self.cellsize - 1) as i32,
                (self.cellsize - 1) as i32,
                COLORS[self.kind as usize],
            );
        }
    }

    pub fn move_by(&mut self, rows: isize, colls: isize) {
        self.row_offset += rows;
        self.coll_offset += colls;
    }

    pub fn rotate_right(&mut self) {
        self.rotation_state = (self.rotation_state + 1) % 4;
    }

    pub fn rotate_left(&mut self) {
        self.rotation_state = (self.rotation_state + 3) % 4;
    }

    pub fn get_cell_position(&self) -> impl Iterator<Item = (usize, usize)> {
        self.cells[self.rotation_state].iter().map(|&(row, col)| {
            (
                (col as isize + self.coll_offset) as usize,
                (row as isize + self.row_offset) as usize,
            )
        })
    }
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum BlockKind {
    L = 1,
    J,
    I,
    O,
    S,
    T,
    Z,
}

impl BlockKind {
    fn cells(&self) -> [&'static [(usize, usize)]; 4] {
        match self {
            Self::L => [
                &[(0, 2), (1, 0), (1, 1), (1, 2)],
                &[(0, 1), (1, 1), (2, 1), (2, 2)],
                &[(1, 0), (1, 1), (1, 2), (2, 0)],
                &[(0, 0), (0, 1), (1, 1), (2, 1)],
            ],

            Self::J => [
                &[(0, 0), (1, 0), (1, 1), (1, 2)],
                &[(0, 1), (0, 2), (1, 1), (2, 1)],
                &[(1, 0), (1, 1), (1, 2), (2, 2)],
                &[(0, 1), (1, 1), (2, 0), (2, 1)],
            ],

            Self::I => [
                &[(1, 0), (1, 1), (1, 2), (1, 3)],
                &[(0, 2), (1, 2), (2, 2), (3, 2)],
                &[(2, 0), (2, 1), (2, 2), (2, 3)],
                &[(0, 1), (1, 1), (2, 1), (3, 1)],
            ],

            Self::O => [
                &[(0, 0), (0, 1), (1, 0), (1, 1)],
                &[(0, 0), (0, 1), (1, 0), (1, 1)],
                &[(0, 0), (0, 1), (1, 0), (1, 1)],
                &[(0, 0), (0, 1), (1, 0), (1, 1)],
            ],

            Self::S => [
                &[(0, 1), (0, 2), (1, 0), (1, 1)],
                &[(0, 1), (1, 1), (1, 2), (2, 2)],
                &[(1, 1), (1, 2), (2, 0), (2, 1)],
                &[(0, 0), (1, 0), (1, 1), (2, 1)],
            ],

            Self::T => [
                &[(0, 1), (1, 0), (1, 1), (1, 2)],
                &[(0, 1), (1, 1), (1, 2), (2, 1)],
                &[(1, 0), (1, 1), (1, 2), (2, 1)],
                &[(0, 1), (1, 0), (1, 1), (2, 1)],
            ],

            Self::Z => [
                &[(0, 0), (0, 1), (1, 1), (1, 2)],
                &[(0, 2), (1, 1), (1, 2), (2, 1)],
                &[(1, 0), (1, 1), (2, 1), (2, 2)],
                &[(0, 1), (1, 0), (1, 1), (2, 0)],
            ],
        }
    }

    pub fn random() -> Self {
        Self::from(rand::random_range(1..=7))
    }
}

impl From<usize> for BlockKind {
    fn from(value: usize) -> Self {
        match value {
            1 => BlockKind::L,
            2 => BlockKind::J,
            3 => BlockKind::I,
            4 => BlockKind::O,
            5 => BlockKind::S,
            6 => BlockKind::T,
            7 => BlockKind::Z,
            _ => unreachable!("invalid"),
        }
    }
}
