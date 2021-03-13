use bevy::prelude::*;

pub struct GameState {
    pub num_pieces: usize,
    pub screen_dims: Vec2,
    pub mouse_pos: Vec2,
    pub hovered_entity: Option<Entity>,
    pub selected_entity0: Option<Entity>,
    pub selected_entity: Option<Entity>,
    pub image_dims: Vec2,
    pub image_path: String,
    pub image_credits: String,
    pub image_url: String,
    pub material_handle: Option<Handle<ColorMaterial>>,
    pub stroked_material_handle: Option<Handle<ColorMaterial>>,
}

pub struct MyEvent;

pub struct EventTriggerState {
    pub event_timer: Timer,
}

impl Default for EventTriggerState {
    fn default() -> Self {
        EventTriggerState {
            event_timer: Timer::from_seconds(1.0, true),
        }
    }
}
