use bevy::prelude::*;

pub struct GameState {
    pub screen_dims: Vec2,
    pub mouse_pos: Vec2,
    pub hovered_entity: Option<Entity>,
    pub selected_entity: Option<Entity>,
    pub image_dims: Vec2,
    pub image_path: String,
    pub image_credits: String,
    pub image_url: String,
    pub material_handle: Option<Handle<ColorMaterial>>,
    pub stroked_material_handle: Option<Handle<ColorMaterial>>,
}
