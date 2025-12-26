mod block;
mod colors;
mod grid;

use raylib::{
    RaylibBuilder, RaylibHandle, RaylibThread, color::Color, ffi::KeyboardKey, prelude::RaylibDraw,
};

use block::Block;
use block::BlockKind;
use grid::Grid;

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    grid: Grid,
    current_block: Block,
    last_update_time: f64,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let (rl, thread) = RaylibBuilder::default().size(300, 600).build();

        Self {
            rl,
            thread,
            grid: Grid::new(),
            current_block: Block::new(BlockKind::random()),
            last_update_time: 0.,
            game_over: false,
        }
    }

    fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        self.grid.draw(&mut d);
        self.current_block.draw(&mut d);

        d.clear_background(Color::WHITE);
    }

    fn update(&mut self) {
        self.handle_input();
        if self.event_triggered(0.2) {
            self.move_block_down();
        }

        if self.game_over {
            self.reset();
        }
    }

    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            self.update();
            self.render();
        }
    }

    fn handle_input(&mut self) {
        let Some(key) = self.rl.get_key_pressed() else {
            return;
        };

        match key {
            KeyboardKey::KEY_LEFT => self.move_block_left(),
            KeyboardKey::KEY_RIGHT => self.move_block_right(),
            KeyboardKey::KEY_UP => self.current_block.rotate_right(),
            KeyboardKey::KEY_DOWN => self.current_block.rotate_left(),
            KeyboardKey::KEY_SPACE => self.hard_drop(),
            _ => {}
        }
    }

    fn move_block_left(&mut self) {
        self.current_block.move_by(0, -1);
        if self.is_block_outside() || !self.block_fits() {
            self.current_block.move_by(0, 1);
        }
    }

    fn move_block_right(&mut self) {
        self.current_block.move_by(0, 1);
        if self.is_block_outside() || !self.block_fits() {
            self.current_block.move_by(0, -1);
        }
    }

    fn move_block_down(&mut self) {
        self.current_block.move_by(1, 0);
        if self.is_block_outside() || !self.block_fits() {
            self.current_block.move_by(-1, 0);
            self.lock_block();
        }
    }

    fn is_block_outside(&self) -> bool {
        for (col, row) in self.current_block.get_cell_position() {
            if self.grid.is_cell_outside(row, col) {
                return true;
            }
        }

        false
    }

    fn lock_block(&mut self) {
        for (col, row) in self.current_block.get_cell_position() {
            self.grid.grid[row][col] = self.current_block.kind as usize;
        }

        self.grid.clear_full_rows();
        self.current_block = Block::new(BlockKind::random());
        if !self.block_fits() {
            self.game_over = true;
        }
    }

    fn event_triggered(&mut self, interval: f64) -> bool {
        let time = self.rl.get_time();

        if time - self.last_update_time >= interval {
            self.last_update_time = time;
            return true;
        }

        false
    }

    fn block_fits(&self) -> bool {
        for (col, row) in self.current_block.get_cell_position() {
            if !self.grid.is_cell_empty(row, col) {
                return false;
            }
        }

        true
    }

    fn reset(&mut self) {
        self.grid = Grid::new();
        self.current_block = Block::new(BlockKind::random());
        self.game_over = false;
    }

    fn hard_drop(&mut self) {
        while !self.is_block_outside() && self.block_fits() {
            self.current_block.move_by(1, 0);
        }
        self.current_block.move_by(-1, 0);
        self.lock_block();
    }
}
