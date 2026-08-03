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

    for x in 0..GRID_W {
        for y in 0..GRID_H {
            cells.push(Cell {
                x,
                y,
                state: CellState::Empty,
            });
        }
    }

    // println!("{:#?}", cells);

    loop {
        clear_background(BLACK);

        let cell_size = (screen_width() / GRID_W as f32).min(screen_height() / GRID_H as f32);

        // draw grid
        //
        for cell in &cells {
            let color = match cell.state {
                CellState::Empty => LIGHTGRAY,
                CellState::Start => GREEN,
                CellState::Goal => YELLOW,
                CellState::Wall => BROWN,
                CellState::Path => BLUE,
            };
            draw_rectangle(
                cell.x as f32 * cell_size,
                cell.y as f32 * cell_size,
                cell_size - 1.0,
                cell_size - 1.0,
                color,
            );
        }

        draw_text("PathFinder", 20.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}
