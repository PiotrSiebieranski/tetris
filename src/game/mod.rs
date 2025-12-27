mod block;
mod colors;
mod grid;

use raylib::prelude::RaylibDrawHandle;
use raylib::window::WindowState;
use raylib::{
    RaylibBuilder, RaylibHandle, RaylibThread, color::Color, ffi::KeyboardKey, prelude::RaylibDraw,
};

use block::Block;
use block::BlockKind;
use grid::Grid;

pub trait Entity {
    fn draw(&self, d: &mut RaylibDrawHandle);
}

#[derive(Default)]
struct Stats {
    score: u32,
    level: u32,
    lines: u32,
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Score: {} | Level: {}", self.score, self.level)
    }
}

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    grid: Grid,
    current_block: Block,
    last_update_time: f64,
    game_over: bool,
    stats: Stats,
}

impl Game {
    pub fn new() -> Self {
        let (mut rl, thread) = RaylibBuilder::default()
            .log_level(raylib::ffi::TraceLogLevel::LOG_NONE)
            .title("Tetris")
            .size(300, 600)
            .build();

        rl.clear_window_state(WindowState::default().set_window_resizable(false));

        Self {
            rl,
            thread,
            grid: Grid::new(),
            current_block: Block::new(BlockKind::random()),
            last_update_time: 0.,
            game_over: false,
            stats: Stats::default(),
        }
    }

    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            self.update();
            self.render();
        }
    }

    fn render(&mut self) {
        self.rl
            .set_window_title(&self.thread, &format!("Tetris | {}", self.stats));

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

    fn handle_input(&mut self) {
        let Some(key) = self.rl.get_key_pressed() else {
            return;
        };

        use KeyboardKey::*;
        match key {
            KEY_LEFT | KEY_H => self.move_block_left(),
            KEY_RIGHT | KEY_L => self.move_block_right(),
            KEY_UP | KEY_K => self.rotate_block_right(),
            KEY_DOWN | KEY_J => self.rotate_block_left(),
            KEY_SPACE => self.hard_drop(),
            KEY_R => self.reset(),
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

    fn rotate_block_right(&mut self) {
        self.current_block.rotate_right();
        if self.is_block_outside() || !self.block_fits() {
            self.current_block.rotate_left();
        }
    }

    fn rotate_block_left(&mut self) {
        self.current_block.rotate_left();
        if self.is_block_outside() || !self.block_fits() {
            self.current_block.rotate_right();
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

        let cleared = self.grid.clear_full_rows();
        self.add_score(cleared as u32);

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

    fn hard_drop(&mut self) {
        while !self.is_block_outside() && self.block_fits() {
            self.current_block.move_by(1, 0);
        }

        self.current_block.move_by(-1, 0);
        self.lock_block();
    }

    fn reset(&mut self) {
        self.grid = Grid::new();
        self.current_block = Block::new(BlockKind::random());
        self.game_over = false;
        self.stats = Stats::default();
    }

    fn add_score(&mut self, cleared: u32) {
        let table = [0, 40, 100, 300, 1200];
        self.stats.score += table[cleared as usize] * (self.stats.level + 1);
        self.stats.lines += cleared;
        self.stats.level = self.stats.lines / 10;
    }
}
