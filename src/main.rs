use macroquad::prelude::*;
use pathfinder::{
    AppState,
    cell::{Cell, CellState},
};

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
    //
    let mut app_state = AppState::new();

    let mut help_text = false;

    loop {
        app_state.detect_key();

        clear_background(BLACK);

        let cell_size = (screen_width() / GRID_W as f32).min(screen_height() / GRID_H as f32);

        // draw grid
        //
        for cell in &cells {
            draw_rectangle(
                cell.width() as f32 * cell_size,
                cell.height() as f32 * cell_size,
                cell_size - 1.0,
                cell_size - 1.0,
                cell.state().color(),
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

            if let Some(select_state) = app_state.get_state()
                && is_mouse_button_down(MouseButton::Left)
            {
                match select_state {
                    CellState::Empty => cells[index].set_state(CellState::Empty),
                    CellState::Start => cells[index].set_state(CellState::Start),
                    CellState::Goal => cells[index].set_state(CellState::Goal),
                    CellState::Wall => cells[index].set_state(CellState::Wall),
                    _ => {}
                }
            }

            if cells[index].state() == &CellState::Empty {}
        }

        draw_text("PathFinder", 20.0, 20.0, 30.0, WHITE);

        if is_key_released(KeyCode::H) {
            help_text = !help_text;
        }
        if help_text {
            draw_text(
                "Help",
                screen_width() / 2.0 - 38.0 / 2.0,
                screen_height() / 2.0,
                38.0,
                DARKBLUE,
            );

            draw_text(
                "H -- Help",
                screen_width() / 2.0 - 38.0 / 2.0,
                screen_height() / 2.0 + 30.0,
                28.0,
                DARKBLUE,
            );

            draw_text(
                "G -- Set Goal Area",
                screen_width() / 2.0 - 38.0 / 2.0,
                screen_height() / 2.0 + 60.0,
                28.0,
                DARKBLUE,
            );

            draw_text(
                "S -- Set Start Area",
                screen_width() / 2.0 - 38.0 / 2.0,
                screen_height() / 2.0 + 90.0,
                28.0,
                DARKBLUE,
            );

            draw_text(
                "W -- Build Wall",
                screen_width() / 2.0 - 38.0 / 2.0,
                screen_height() / 2.0 + 120.0,
                28.0,
                DARKBLUE,
            );
        }

        next_frame().await;
    }
}
