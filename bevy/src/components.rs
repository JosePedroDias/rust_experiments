use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct TileData {
    pub index: usize,
    pub original_index: usize,
    pub center: Vec3,
    pub dims: Vec2,
    pub uvs: (f32, f32, f32, f32),
}

#[derive(Debug)]
pub struct Animate {
    pub start_t: f64,
    pub duration: f64,
    pub kind: AnimateKind,
    pub kill_ent_at_end: bool,
}

#[derive(Debug)]
pub enum AnimateKind {
    SHRINK = 1,
    GROW,
}

pub struct ElapsedTime;

pub struct Credits;

pub struct MainCamera;

pub struct StrokedTile;
