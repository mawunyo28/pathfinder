use macroquad::prelude::*;

const GRID_W: usize = 40;

const GRID_H: usize = 30;

#[derive(Debug, Clone)]
struct Cell {
    x: usize,
    y: usize,
    state: CellState,
}

#[derive(Debug, Clone)]
enum CellState {
    Empty,
    Start,
    Goal,
    Wall,
    Path,
}

#[macroquad::main("Pathfinder")]
async fn main() {
    let mut cells: Vec<Cell> = Vec::with_capacity(GRID_W * GRID_H);

    for x in 1..=GRID_W {
        for y in 1..=GRID_H {
            cells.push(Cell {
                x,
                y,
                state: CellState::Empty,
            });
        }
    }

    println!("{:#?}", cells);

    loop {
        clear_background(BLACK);

        let cell_size = (screen_width() / GRID_W as f32).min(screen_height() / GRID_H as f32);

        // draw grid
        //
        for cell in cells.clone() {}

        draw_text("PathFinder", 20.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}
