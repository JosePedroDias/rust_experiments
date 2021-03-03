use glam::{vec2, vec3};
use macroquad::models::Vertex;
use macroquad::prelude::*;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

type FPair = (f32, f32);

type PMPair = (Piece, Mesh);

fn window_conf() -> Conf {
    Conf {
        window_title: "focus point".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

// 0 1
// 2 3
fn quad(
    xi: f32,
    yi: f32,
    xf: f32,
    yf: f32,
    ui: f32,
    vi: f32,
    uf: f32,
    vf: f32,
    texture: &Texture2D,
    color: &Color,
) -> Mesh {
    Mesh {
        vertices: vec![
            Vertex {
                position: vec3(xi, yi, 0.0),
                uv: vec2(ui, vi),
                color: *color,
            },
            Vertex {
                position: vec3(xf, yi, 0.0),
                uv: vec2(uf, vi),
                color: *color,
            },
            Vertex {
                position: vec3(xi, yf, 0.0),
                uv: vec2(ui, vf),
                color: *color,
            },
            Vertex {
                position: vec3(xf, yf, 0.0),
                uv: vec2(uf, vf),
                color: *color,
            },
        ],
        indices: vec![1, 0, 2, 1, 2, 3],
        texture: Some(*texture),
    }
}

fn quadi(p: &Piece, texture: &Texture2D, color: &Color) -> Mesh {
    quad(
        p.x0, p.y0, p.x1, p.y1, p.u0, p.v0, p.u1, p.v1, texture, color,
    )
}

const N: usize = 30;

#[derive(Clone, Debug)]
struct Piece {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    u0: f32,
    u1: f32,
    v0: f32,
    v1: f32,
}

//#[derive(Debug)]
//Mesh

fn lerp(a: f32, b: f32, i: f32) -> f32 {
    i * b + (1.0 - i) * a
}

fn split(p: Piece, axis_index: usize, r: f32) -> (Piece, Piece) {
    if axis_index == 0 {
        (
            Piece {
                x0: p.x0,
                x1: lerp(p.x0, p.x1, r),
                y0: p.y0,
                y1: p.y1,
                u0: p.u0,
                u1: lerp(p.u0, p.u1, r),
                v0: p.v0,
                v1: p.v1,
            },
            Piece {
                x0: lerp(p.x0, p.x1, r),
                x1: p.x1,
                y0: p.y0,
                y1: p.y1,
                u0: lerp(p.u0, p.u1, r),
                u1: p.u1,
                v0: p.v0,
                v1: p.v1,
            },
        )
    } else {
        (
            Piece {
                x0: p.x0,
                x1: p.x1,
                y0: p.y0,
                y1: lerp(p.y0, p.y1, r),
                u0: p.u0,
                u1: p.u1,
                v0: p.v0,
                v1: lerp(p.v0, p.v1, r),
            },
            Piece {
                x0: p.x0,
                x1: p.x1,
                y0: lerp(p.y0, p.y1, r),
                y1: p.y1,
                u0: p.u0,
                u1: p.u1,
                v0: lerp(p.v0, p.v1, r),
                v1: p.v1,
            },
        )
    }
}

fn random_color() -> Color {
    Color {
        r: rand::gen_range(0.33, 1.0),
        g: rand::gen_range(0.33, 1.0),
        b: rand::gen_range(0.33, 1.0),
        a: 1.0,
    }
}

fn find_pair(pairs: &Vec<(Piece, Mesh)>, point: FPair) -> Option<usize> {
    let mut nearest_index: Option<usize> = None;
    let mut nearest_dist: f32 = f32::MAX;
    let mut i: usize = 0;
    for (p, _) in pairs.iter() {
        let dx = point.0 - (p.x1 - p.x0) / 2.0;
        let dy = point.1 - (p.y1 - p.y0) / 2.0;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < nearest_dist {
            nearest_index = Some(i);
            nearest_dist = dist;
        }
        i += 1;
    }
    return nearest_index;
}

fn replace_uvs_updating_meshes(p1: &mut PMPair, p2: &mut PMPair, texture: &Texture2D) {
    mem::swap(&mut p1.0.u0, &mut p2.0.u0);
    mem::swap(&mut p1.0.u1, &mut p2.0.u1);
    mem::swap(&mut p1.0.v0, &mut p2.0.v0);
    mem::swap(&mut p1.0.v1, &mut p2.0.v1);
    p1.1 = quadi(&p1.0, &texture, &random_color());
    p2.1 = quadi(&p2.0, &texture, &random_color());
}

#[macroquad::main(window_conf())]
async fn main() {
    let texture: Texture2D = load_texture("mosque.png").await;

    let w = screen_width();
    let h = screen_height();
    let ar = w / h;

    let iw = texture.width();
    let ih = texture.height();
    let iar = iw / ih;

    let s: f32 = if iar > ar { w / iw } else { h / ih };

    let ix0: f32 = (w - iw * s) / 2.0;
    let iy0: f32 = (h - ih * s) / 2.0;
    let ix1: f32 = (w + iw * s) / 2.0;
    let iy1: f32 = (h + ih * s) / 2.0;

    let seed = Piece {
        x0: ix0,
        x1: ix1,
        y0: iy0,
        y1: iy1,
        u0: 0.0,
        u1: 1.0,
        v0: 0.0,
        v1: 1.0,
    };

    let seed_num: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    rand::srand(seed_num);
    let mut pieces = vec![seed];

    let mut step = 0;

    loop {
        //let one = pieces.pop().unwrap();
        let one = pieces.remove(rand::gen_range(0, pieces.len()));

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
            println!("position: {:?}", p);
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
