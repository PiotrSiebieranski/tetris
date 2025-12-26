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
    blocks: Vec<Block>,
    current_block: Block,
}

impl Game {
    pub fn new() -> Self {
        let (rl, thread) = RaylibBuilder::default().size(300, 600).build();

        Self {
            rl,
            thread,
            grid: Grid::new(),
            blocks: Vec::new(),
            current_block: Block::new(BlockKind::random()),
        }
    }

    fn render(&mut self) {
        self.update();

        let mut d = self.rl.begin_drawing(&self.thread);
        self.grid.draw(&mut d);

        for block in &self.blocks {
            block.draw(&mut d);
        }

        self.current_block.draw(&mut d);

        d.clear_background(Color::DARKBLUE);
    }

    fn update(&mut self) {
        self.handle_input();
    }

    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
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
            _ => {}
        }
    }

    fn move_block_left(&mut self) {
        self.current_block.move_by(0, -1);
        if self.is_block_outside() {
            self.current_block.move_by(0, 1)
        }
    }

    fn move_block_right(&mut self) {
        self.current_block.move_by(0, 1);
        if self.is_block_outside() {
            self.current_block.move_by(0, -1)
        }
    }

    fn move_block_down(&mut self) {
        self.current_block.move_by(1, 0);
        if self.is_block_outside() {
            self.current_block.move_by(-1, 0)
        }
    }

    fn is_block_outside(&self) -> bool {
        for tile in self.current_block.get_cell_position() {
            if self.grid.is_cell_outside(tile.0, tile.1) {
                return true;
            }
        }
        false
    }
}
