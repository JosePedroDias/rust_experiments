use bevy::prelude::*;

pub struct MainCamera;

#[derive(Clone, Copy)]
pub struct TileData {
    pub index: usize,
    pub original_index: usize,
    pub center: Vec3,
    pub dims: Vec2,
    pub uvs: (f32, f32, f32, f32),
}

pub struct Credits;

pub struct StrokedTile;
