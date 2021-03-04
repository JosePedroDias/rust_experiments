use macroquad::models::{Vertex, Mesh};
use macroquad::color::Color;
use macroquad::texture::Texture2D;
use macroquad::rand::gen_range;
use std::mem;

use crate::{vec2, vec3};

pub type FPair = (f32, f32);
pub type PMPair = (Piece, Mesh);

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

#[derive(Clone, Debug)]
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
        r: gen_range(0.33, 1.0),
        g: gen_range(0.33, 1.0),
        b: gen_range(0.33, 1.0),
        a: 1.0,
    }
}

pub fn find_pair(pairs: &Vec<(Piece, Mesh)>, point: FPair) -> Option<usize> {
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

pub fn replace_uvs_updating_meshes(p1: &mut PMPair, p2: &mut PMPair, texture: &Texture2D) {
    mem::swap(&mut p1.0.u0, &mut p2.0.u0);
    mem::swap(&mut p1.0.u1, &mut p2.0.u1);
    mem::swap(&mut p1.0.v0, &mut p2.0.v0);
    mem::swap(&mut p1.0.v1, &mut p2.0.v1);
    p1.1 = quadi(&p1.0, &texture, &random_color());
    p2.1 = quadi(&p2.0, &texture, &random_color());
}
