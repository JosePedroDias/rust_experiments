mod types;

pub use glam::*;

use crate::types::*;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "focus point".to_owned(),
        window_width: 1280,
        window_height: 768,
        //window_width: 2000,
        //window_height: 1500,
        //window_width: 3072,
        //window_height: 1920,
        //fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    seed_with_clock();

    let image_path = elect_image().await;
    let texture: Texture2D = load_texture(&image_path[..]).await;

    let w = screen_width();
    let h = screen_height();

    let mut puzzle = generate_puzzle(w, h, &texture);

    let mut last_selected_index: Option<usize> = None;
    let mut _hovered_index: Option<usize> = None; // TODO WHY THE WARNING??

    loop {
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        let p = mouse_position();
        _hovered_index = find_full_piece(&puzzle, p);

        if is_mouse_button_released(MouseButton::Left) {
            let idx = find_full_piece(&puzzle, p).unwrap(); // TODO HANDLE NONE
            if let Some(prev_idx) = last_selected_index {
                if prev_idx != idx {
                    swap_pieces(&mut puzzle, prev_idx, idx, &texture);
                    last_selected_index = None;
                }
            } else {
                last_selected_index = Some(idx);
            }

            if is_puzzle_solved(&puzzle) {
                println!("PUZZLE SOLVED!");
            }
        }

        clear_background(BLACK);

        set_default_camera();

        for (_, m, _) in puzzle.iter() {
            draw_mesh(&m);
        }

        if let Some(lsi) = _hovered_index {
            let q = puzzle[lsi].0;
            rect_wireframe(&q);                
        }

        // TO PREVENT TINTING TO NEXT QUAD?
        draw_line(0.0, 0.0, 0.0, 0.0, 1.0, WHITE);

        next_frame().await
    }
}
