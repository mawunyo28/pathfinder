use std::collections::{BinaryHeap, HashMap, HashSet};

use macroquad::prelude::*;
use pathfinder::{
    AppState,
    cell::{Cell, CellState},
};

const GRID_W: usize = 40;

const GRID_H: usize = 30;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
struct NodeCost {
    pos: (usize, usize),
    f: i32,
}

#[macroquad::main("Pathfinder")]
async fn main() {
    let mut cells: Vec<Cell> = Vec::with_capacity(GRID_W * GRID_H);

    let mut open: BinaryHeap<NodeCost> = BinaryHeap::new();

    let mut closed: HashSet<(usize, usize)> = HashSet::new();
    let mut g_score: HashMap<(usize, usize), i32> = HashMap::new();

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut start: (usize, usize);

    let mut goal: (usize, usize) = (0, 0); // would change to option later

    let mut solve = false;

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
                && is_mouse_button_released(MouseButton::Left)
            {
                match select_state {
                    CellState::Empty => cells[index].set_state(CellState::Empty),
                    CellState::Start => {
                        start = (gx, gy);

                        // println!("{:?}", start);

                        open.push(NodeCost { pos: start, f: 0 });

                        println!("{:?}", open);
                        g_score.insert(start, 0);
                        cells[index].set_state(CellState::Start);
                    }
                    CellState::Goal => {
                        goal = (gx, gy);
                        cells[index].set_state(CellState::Goal);
                    }
                    CellState::Wall => cells[index].set_state(CellState::Wall),
                    _ => {}
                }
            }

            if cells[index].state() == &CellState::Empty {}
        }

        // A* algoritm
        //
        //
        //
        if is_key_released(KeyCode::Space) {
            solve = !solve;
        }

        if solve {
            let current = open.pop().unwrap().pos;

            if current == goal {
                let mut path = vec![current];

                let mut node = current;

                while let Some(&prev) = came_from.get(&node) {
                    path.push(prev);
                    node = prev;
                }

                path.reverse();

                for pos in &path {
                    let index = pos.0 * GRID_H + pos.1;

                    if *cells[index].state() != CellState::Start
                        && *cells[index].state() != CellState::Goal
                    {
                        cells[index].set_state(CellState::Path);
                    }
                }
            } else {
                let index = current.0 * GRID_H + current.1;

                if *cells[index].state() != CellState::Start {
                    cells[index].set_state(CellState::Empty);
                }
            }

            for neighbour in neighbors_of(current, &cells) {
                let neighbour = neighbour;

                let index = neighbour.0 * GRID_H + neighbour.1;

                if *cells[index].state() == CellState::Wall {
                    continue;
                }

                if closed.contains(&neighbour) {
                    continue;
                }

                let tentative_g = g_score[&current] + step_cost(current, neighbour);

                let is_better = match g_score.get(&neighbour) {
                    Some(&existing_g) => tentative_g < existing_g,
                    None => true,
                };

                if is_better {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g);

                    let h = heuristic(neighbour, goal);

                    let f = tentative_g + h;

                    open.push(NodeCost { pos: neighbour, f });
                }
            }

            closed.insert(current);
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

fn step_cost(current: (usize, usize), neighbour: (usize, usize)) -> i32 {
    if current.0 != neighbour.0 && neighbour.1 != current.1 {
        14
    } else {
        10
    }
}

fn heuristic(pos: (usize, usize), goal: (usize, usize)) -> i32 {
    let dx = (pos.0 as i32 - goal.0 as i32).abs();
    let dy = (pos.1 as i32 - goal.1 as i32).abs();

    let straight = 10;
    let diagonal = 14;

    straight * (dx + dy) + (diagonal - 2 * straight) * dx.min(dy)
}

fn neighbors_of(pos: (usize, usize), cells: &Vec<Cell>) -> Vec<Option<(usize, usize)>> {
    let index = pos.0 * GRID_H + pos.1;

    let index_top_left = index - GRID_H - 1;
    let index_top_mid = index - GRID_H;
    let index_top_right = index - GRID_H + 1;
    let index_left = index - 1;
    let index_right = index + 1;
    let index_btm_left = index + GRID_H - 1;
    let index_btm_mid = index + GRID_H;
    let index_btm_right = index + GRID_H + 1;

    let top_left = match cells.get(index_top_left) {
        Some(c) => Some((c.width(), c.height())),
        None => None,
    };

    let top_mid = match cells.get(index_top_mid) {
        Some(c) => Some((c.width(), c.height())),
        None => None,
    };

    let top_right = match cells.get(index_top_right) {
        Some(c) => Some((c.width(), c.height())),
        None => None,
    };

    let left = match cells.get(index_left) {
        Some(c) => Some((c.width(), c.height())),
        None => None,
    };

    let right = match cells.get(index_right) {
        Some(c) => Some((c.width(), c.height())),
        None => None,
    };

    let btm_left = match cells.get(index_btm_left) {
        Some(c) => Some((c.width(), c.height())),
        None => None,
    };

    let btm_mid = match cells.get(index_btm_mid) {
        Some(c) => Some((c.width(), c.height())),
        None => None,
    };

    let btm_right = match cells.get(index_btm_right) {
        Some(c) => Some((c.width(), c.height())),
        None => None,
    };

    let neighbors: Vec<Option<(usize, usize)>> = vec![
        top_left, top_mid, top_right, left, right, btm_left, btm_mid, btm_right,
    ];

    neighbors
}
