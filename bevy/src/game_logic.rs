use bevy::{prelude::*, render::mesh::Mesh};

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
