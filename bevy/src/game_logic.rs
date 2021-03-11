use bevy::{prelude::*, render::mesh::Mesh};

use super::components::TileData;
use super::random::*;
use std::mem;

pub const W: usize = 10;
pub const H: usize = 8;

//const FONT:&str = "fonts/FiraSans-Bold.ttf";
pub const FONT: &str = "fonts/FiraMono-Medium.ttf";

pub fn image_contain(screen_dims: Vec2, image_dims: Vec2) -> f32 {
    let (w, h) = screen_dims.into();
    let (iw, ih) = image_dims.into();
    let sar = w / h;
    let iar = iw / ih;
    return if iar > sar { w / iw } else { h / ih };
}

pub fn generate_tile_bundle(
    mesh: Handle<Mesh>,
    mat: Handle<ColorMaterial>,
    center: Vec3,
) -> SpriteBundle {
    SpriteBundle {
        mesh: mesh,
        material: mat,
        sprite: Sprite {
            size: Vec2::new(1., 1.),
            resize_mode: SpriteResizeMode::Manual,
        },
        transform: Transform::from_translation(center),
        ..Default::default()
    }
}

fn swap_pieces(puzzle: &mut Vec<TileData>, i0: usize, i1: usize) {
    let mut p1 = puzzle[i0];
    let mut p2 = puzzle[i1];
    mem::swap(&mut p1.index, &mut p2.index);
    mem::swap(&mut p1.center, &mut p2.center);
    mem::swap(&mut p1.uvs, &mut p2.uvs);
}

pub fn shuffle(times: usize, puzzle: &mut Vec<TileData>) {
    let len = puzzle.len();
    for _ in 0..times {
        let mut rng = setup_random_seed();
        let i0 = get_usize(&mut rng, len);
        let i1 = get_usize(&mut rng, len);
        if i0 != i1 {
            swap_pieces(puzzle, i0, i1);
        }
    }
}

pub fn lerp(a: f32, b: f32, i: f32) -> f32 {
    i * b + (1.0 - i) * a
}

fn split_center(center: f32, dim: f32, r: f32) -> f32 {
    center - center * 0.5 + lerp(0., dim, r)
}

pub fn split(p: TileData, axis_index: usize, r: f32) -> (TileData, TileData) {
    if axis_index == 0 {
        (
            TileData {
                index: 0,
                original_index: 0,
                center: Vec3::new(split_center(p.center.x, p.dims.x, r), p.center.y, 0.),
                dims: Vec2::new(lerp(0., p.dims.x, r), p.dims.y),
                uvs: (p.uvs.0, lerp(p.uvs.0, p.uvs.1, r), p.uvs.2, p.uvs.3),
            },
            TileData {
                index: 0,
                original_index: 0,
                center: Vec3::new(split_center(p.center.x, p.dims.x, 1. - r), p.center.y, 0.),
                dims: Vec2::new(lerp(0., p.dims.x, 1. - r), p.dims.y),
                uvs: (lerp(p.uvs.0, p.uvs.1, r), p.uvs.1, p.uvs.2, p.uvs.3),
            },
        )
    } else {
        (
            TileData {
                index: 0,
                original_index: 0,
                center: Vec3::new(p.center.x, split_center(p.center.y, p.dims.y, r), 0.),
                dims: Vec2::new(p.dims.x, lerp(0., p.dims.y, r)),
                uvs: (p.uvs.0, p.uvs.1, p.uvs.2, lerp(p.uvs.0, p.uvs.3, r)),
            },
            TileData {
                index: 0,
                original_index: 0,
                center: Vec3::new(p.center.x, split_center(p.center.y, p.dims.y, 1. - r), 0.),
                dims: Vec2::new(p.dims.x, lerp(0., p.dims.y, 1. - r)),
                uvs: (p.uvs.0, p.uvs.1, lerp(p.uvs.2, p.uvs.3, r), p.uvs.3),
            },
        )
    }
}

/* fn heuristic_random(pieces:&Vec<Piece>) -> usize {
    gen_range(0, pieces.len()-1)
} */

fn heuristic_having_largest_area(pieces: &Vec<TileData>) -> usize {
    let mut bigger_area = 0.0f32;
    let mut bigger_i: usize = 0;
    for (i, p) in pieces.iter().enumerate() {
        let area = p.dims.x * p.dims.y;
        if area > bigger_area {
            bigger_area = area;
            bigger_i = i;
        }
    }
    return bigger_i;
}

const N: usize = 30;

pub fn generate_puzzle(dims: Vec2) -> Vec<TileData> {
    let mut rng = setup_random_seed();

    let seed_piece = TileData {
        index: 0,
        original_index: 0,
        center: Vec3::new(0., 0., 0.),
        dims: dims.clone(),
        uvs: (0., 1., 0., 1.),
    };
    let mut puzzle = vec![seed_piece];

    let mut step = 0;

    loop {
        let one = puzzle.remove(heuristic_having_largest_area(&puzzle));

        let index = if one.dims.x > one.dims.y { 0 } else { 1 };
        let ratio = get_f32(&mut rng) * 0.6 + 0.2;
        let (two, three) = split(one, index, ratio);
        puzzle.push(two);
        puzzle.push(three);

        step += 1;
        if step == N {
            break;
        }
    }

    for (i, td) in puzzle.iter_mut().enumerate() {
        td.original_index = i;
        td.index = i;
    }

    shuffle(N, &mut puzzle);

    puzzle
}
