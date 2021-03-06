mod types;
mod shaders;

pub use glam::*;

use crate::types::*;
use crate::shaders::*;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "focus point".to_owned(),
        //window_width: 1280,
        //window_height: 768,
        fullscreen: true,
        //high_dpi: true,
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

    let translucent_mat = create_material();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        let p = mouse_position();
        _hovered_index = find_full_piece(&puzzle, p);

        if is_mouse_button_released(MouseButton::Left) {
            let idx = find_full_piece(&puzzle, p);
            if let Some(idx) = idx {
                if let Some(prev_idx) = last_selected_index {
                    if prev_idx != idx {
                        swap_pieces(&mut puzzle, prev_idx, idx, &texture);
                        last_selected_index = None;
                    }
                } else {
                    last_selected_index = Some(idx);
                }
            }

            if is_puzzle_solved(&puzzle) {
                println!("PUZZLE SOLVED!");
            }
        }

        clear_background(BLACK);

        set_default_camera();

        for (i, (_, m, _)) in puzzle.iter().enumerate() {
            let mut is_translucent = false;

            if _hovered_index.is_some() {
                if _hovered_index.unwrap() == i {
                    is_translucent = true;
                }
            }

            if is_translucent {
                gl_use_material(translucent_mat);
                let ratio = ((get_time()*0.5) % 1.0).sin() as f32;
                //let ratio = ((get_time()*3.0).sin() % 3.1415927) as f32;
                //println!("{:.3}", ratio);
                translucent_mat.set_uniform("Ratio", ratio);
            }

            draw_mesh(&m);

            if is_translucent {
                gl_use_default_material();
            }
        }

        if let Some(lsi) = _hovered_index {
            let q = puzzle[lsi].0;
            rect_wireframe(&q);                
        }

        let time = &get_elapsed_time(get_time())[..];
        draw_text(time, 32.0, 52.0, 48.0, BLACK);
        draw_text(time, 30.0, 50.0, 48.0, WHITE);

        next_frame().await
    }
}
