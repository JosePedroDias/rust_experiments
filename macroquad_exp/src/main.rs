mod types;

pub use glam::*;

use crate::types::*;

use macroquad::prelude::*;
use macroquad::rand::{srand, gen_range};

use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 30;

fn window_conf() -> Conf {
    Conf {
        window_title: "focus point".to_owned(),
        window_width: 1024,
        window_height: 768,
        //loading: |x| println!("LOADING {:?}"),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
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
        let one = pieces.remove(gen_range(0, pieces.len()));

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

    let mut pairs: Vec<(Piece, Mesh)> = pieces
        .iter()
        .map(|p| (p.clone(), quadi(&p, &texture, &random_color())))
        .collect();

    let mut last_selected_index: Option<usize> = None;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        if is_mouse_button_released(MouseButton::Left) {
            let p = mouse_position();
            println!("position: {:.0} {:.0}", p.0, p.1);
            //println!("pairs.len before: {}", pairs.len());
            let idx = find_pair(&pairs, p).unwrap();
            if let Some(prev_idx) = last_selected_index {
                println!("trying to swap #{} #{}", prev_idx, idx);
                if prev_idx != idx {
                    println!("swapping #{} #{}", prev_idx, idx);
                    let mut p1 = pairs.remove(prev_idx);
                    let mut p2 = pairs.remove(idx);
                    println!("piece 1: {:?}", p1.0);
                    println!("piece 2: {:?}", p2.0);
                    replace_uvs_updating_meshes(&mut p1, &mut p2, &texture);
                    pairs.push(p1);
                    pairs.push(p2);
                    last_selected_index = None;
                }
            } else {
                last_selected_index = Some(idx);
            }
            //println!("pairs.len after: {}", pairs.len());
            println!("last_selected_index after: {:?}", last_selected_index);
        }

        clear_background(BLACK);

        set_default_camera();

        for (_, m) in pairs.iter() {
            draw_mesh(&m);
        }

        next_frame().await
    }
}
