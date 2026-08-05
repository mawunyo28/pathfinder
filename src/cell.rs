use std::ops::Not;

use macroquad::color::{BLUE, BROWN, Color, GREEN, LIGHTGRAY, YELLOW};

#[derive(Debug, Clone)]
pub struct Cell {
    x: usize,
    y: usize,
    state: CellState,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Cell {
            x,
            y,
            state: CellState::Empty,
        }
    }

    pub fn state(&self) -> &CellState {
        &self.state
    }

    pub fn width(&self) -> usize {
        self.x
    }

    pub fn height(&self) -> usize {
        self.y
    }

    pub fn set_state(&mut self, state: CellState) {
        self.state = state;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CellState {
    Empty,
    Start,
    Goal,
    Wall,
    Path,
}

impl CellState {
    pub fn color(self) -> Color {
        match self {
            CellState::Empty => LIGHTGRAY,
            CellState::Start => GREEN,
            CellState::Goal => YELLOW,
            CellState::Wall => BROWN,
            CellState::Path => BLUE,
        }
    }
}
