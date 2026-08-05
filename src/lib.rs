use macroquad::input::{KeyCode, is_key_down};

use crate::cell::CellState;

pub mod cell;

// None == Empty
// W == Wall
// G == Goal
// S == Start

pub struct AppState {
    select_state: Option<CellState>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            select_state: Some(CellState::Wall),
        }
    }

    pub fn detect_key(&mut self) {
        if is_key_down(KeyCode::G) {
            self.select_state = Some(CellState::Goal)
        } else if is_key_down(KeyCode::S) {
            self.select_state = Some(CellState::Start);
        } else if is_key_down(KeyCode::W) {
            self.select_state = Some(CellState::Wall)
        } else {
            self.select_state = Some(CellState::Empty)
        }
    }

    pub fn get_state(&self) -> Option<CellState> {
        self.select_state
    }
}
