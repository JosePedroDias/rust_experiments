use macroquad::models::{Vertex, Mesh};
use macroquad::shapes::{draw_rectangle_lines};
use macroquad::color::{Color, WHITE, MAGENTA};
use macroquad::texture::Texture2D;
use macroquad::rand::{srand, gen_range};
use macroquad::file::load_string;
use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;

use crate::{vec2, vec3};

const N: usize = 30;

pub type FPair = (f32, f32);

pub type FullPiece = (Piece, Mesh, usize);

pub async fn elect_image() -> String {
    let res = load_string("./images/images.txt").await;
    let s = res.unwrap();
    let images:Vec<&str> = s.split("\n").collect();
    let image = images[gen_range(0, images.len()-1)];
    return format!("images/{}", image);
}

pub fn seed_with_clock() {
    let seed_num: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    srand(seed_num);
}

pub fn generate_puzzle(w:f32, h: f32, texture: &Texture2D)-> Vec<FullPiece> {
    let iw = texture.width();
    let ih = texture.height();

    let seed = generate_seed_piece(w, h, iw, ih);
    let mut pieces = vec![seed];

    let mut step = 0;

    loop {
        let one = pieces.remove(gen_range(0, pieces.len()-1));

        let w = one.x1 - one.x0;
        let h = one.y1 - one.y0;

        let index = if w > h { 0 } else { 1 };
        let ratio = gen_range(0.2, 0.8);
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

    puzzle
}

pub fn generate_seed_piece(w:f32, h:f32, iw:f32, ih:f32) -> Piece {
    let ar = w / h;
    let iar = iw / ih;

    let s: f32 = if iar > ar { w / iw } else { h / ih };

    let ix0: f32 = (w - iw * s) / 2.0;
    let iy0: f32 = (h - ih * s) / 2.0;
    let ix1: f32 = (w + iw * s) / 2.0;
    let iy1: f32 = (h + ih * s) / 2.0;

    Piece {
        x0: ix0,
        x1: ix1,
        y0: iy0,
        y1: iy1,
        u0: 0.0,
        u1: 1.0,
        v0: 0.0,
        v1: 1.0,
    }
}

// 0 1
// 2 3
pub fn quad(
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

pub fn quadi(p: &Piece, texture: &Texture2D, color: &Color) -> Mesh {
    quad(
        p.x0, p.y0, p.x1, p.y1, p.u0, p.v0, p.u1, p.v1, texture, color,
    )
}

pub fn rect_wireframe(p: &Piece) {
    draw_rectangle_lines(p.x0, p.y0, p.x1 - p.x0, p.y1 - p.y0, 6.0, MAGENTA);
}

#[derive(Clone, Debug, Copy)]
pub struct Piece {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub u0: f32,
    pub u1: f32,
    pub v0: f32,
    pub v1: f32,
}

pub fn lerp(a: f32, b: f32, i: f32) -> f32 {
    i * b + (1.0 - i) * a
}

pub fn split(p: Piece, axis_index: usize, r: f32) -> (Piece, Piece) {
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

pub fn random_color() -> Color {
    Color {
        r: gen_range(0.5, 1.0),
        g: gen_range(0.5, 1.0),
        b: gen_range(0.5, 1.0),
        a: 1.0,
    }
}

pub fn swap_pieces(puzzle: &mut Vec<FullPiece>, i0:usize, i1: usize, texture:&Texture2D) {
    //println!("swapping #{} #{}", i0, i1);
    let mut p1 = puzzle[i0].0;
    let mut p2 = puzzle[i1].0;
    swap_piece_uvs(&mut p1, &mut p2);
    puzzle[i0].0 = p1;
    puzzle[i1].0 = p2;
    puzzle[i0].1 = quadi(&p1, &texture, &WHITE); // &random_color());
    puzzle[i1].1 = quadi(&p2, &texture, &WHITE); // &random_color());

    // swap indices without delegating ownership
    let tmp = puzzle[i0].2;
    puzzle[i0].2 = puzzle[i1].2;
    puzzle[i1].2 = tmp;
}

pub fn shuffle(times:usize, puzzle: &mut Vec<FullPiece>, texture:&Texture2D) {
    let max_idx = puzzle.len() - 1;
    for _ in 0..times { 
        let i0 = gen_range(0, max_idx);
        let i1 = gen_range(0, max_idx);
        if i0 != i1 {
            swap_pieces(puzzle, i0, i1, &texture);
        }
    }
}

pub fn find_full_piece(puzzle: &Vec<FullPiece>, point: FPair) -> Option<usize> {
    let (x, y) = point;

    for (i, (p, _, _)) in puzzle.iter().enumerate() {
        if (p.x0..p.x1).contains(&x) && (p.y0..p.y1).contains(&y) {
            return Some(i);
        }
    }
    
    return None;
}

pub fn swap_piece_uvs(p1: &mut Piece, p2: &mut Piece) {
    mem::swap(&mut p1.u0, &mut p2.u0);
    mem::swap(&mut p1.u1, &mut p2.u1);
    mem::swap(&mut p1.v0, &mut p2.v0);
    mem::swap(&mut p1.v1, &mut p2.v1);
}

pub fn is_puzzle_solved(puzzle:&Vec<FullPiece>) -> bool {
    for (i, (_, _, ii)) in puzzle.iter().enumerate() {
        if &i != ii {
            return false;
        }
    }
    return true;
}