mod types;

pub use glam::*;

use crate::types::*;

use macroquad::prelude::*;
use macroquad::rand::{srand, gen_range};

use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 20;

fn window_conf() -> Conf {
    Conf {
        window_title: "focus point".to_owned(),
        //window_width: 1024,
        //window_height: 768,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    window_conf();

    let texture: Texture2D = load_texture("mosque.png").await;

    let w = screen_width();
    let h = screen_height();

    let iw = texture.width();
    let ih = texture.height();

    let seed = generate_seed_piece(w, h, iw, ih);

    let seed_num: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    srand(seed_num);

    let mut pieces = vec![seed];

    let mut step = 0;

    loop {
        let one = pieces.remove(gen_range(0, pieces.len()-1));

        let w = one.x1 - one.x0;
        let h = one.y1 - one.y0;

        let index = if w > h { 0 } else { 1 };
        let ratio = rand::gen_range(0.2, 0.8);
        let (two, three) = split(one, index, ratio);
        pieces.push(two);
        pieces.push(three);

        step += 1;
        if step == N {
            break;
        }
    }

    let mut puzzle: Vec<FullPiece> = pieces
        .iter().enumerate()
        .map(|(i, p)| (p.clone(), quadi(&p, &texture, &random_color()), i))
        .collect();

    shuffle(30, &mut puzzle, &texture);

    let mut last_selected_index: Option<usize> = None;
    let mut _hovered_index: Option<usize> = None; // TODO WHY THE WARNING??

    loop {
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        {
            let p = mouse_position();
            _hovered_index = find_full_piece(&puzzle, p);
        }
        

        if is_mouse_button_released(MouseButton::Left) {
            let p = mouse_position();
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

        next_frame().await
    }
}
