use macroquad::prelude::*;
use pathfinder::cell::{Cell, CellState};

const GRID_W: usize = 40;

const GRID_H: usize = 30;

#[macroquad::main("Pathfinder")]
async fn main() {
    let mut cells: Vec<Cell> = Vec::with_capacity(GRID_W * GRID_H);

    for x in 0..GRID_W {
        for y in 0..GRID_H {
            cells.push(Cell::new(x, y));
        }
    }

    // println!("{:#?}", cells);

    loop {
        clear_background(BLACK);

        let cell_size = (screen_width() / GRID_W as f32).min(screen_height() / GRID_H as f32);

        // draw grid
        //
        for cell in &cells {
            let color = match cell.state() {
                CellState::Empty => LIGHTGRAY,
                CellState::Start => GREEN,
                CellState::Goal => YELLOW,
                CellState::Wall => BROWN,
                CellState::Path => BLUE,
            };
            draw_rectangle(
                cell.width() as f32 * cell_size,
                cell.height() as f32 * cell_size,
                cell_size - 1.0,
                cell_size - 1.0,
                color,
            );
        }

        // get position

        let (mx, my) = mouse_position();

        // println!("Mouse {mx}, {my}");

        let (gx, gy) = ((mx / cell_size) as usize, (my / cell_size) as usize);

        // println!("Mouse {gx}, {gy}");
        //

        // Change state at mouse pos
        //
        if gx < GRID_W && gy < GRID_H {
            let index = gx * GRID_H + gy;

            if is_mouse_button_down(MouseButton::Left) {
                cells[index].set_state(CellState::Wall);
            }
        }

        draw_text("PathFinder", 20.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}
