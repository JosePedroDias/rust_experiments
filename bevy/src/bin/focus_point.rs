use bevy::prelude::*;
use bevy_dev::{game_logic::*, image_metadatas::select_random_image, resources::*, systems::*};

fn main() {
    let screen_dims: Vec2 = Vec2::new(1024., 768.);

    let image_md = select_random_image();
    let image_path = format!("textures/images/{}.jpg", image_md.file_name);
    let scale = image_contain(screen_dims, image_md.dims);
    let image_dims = image_md.dims * scale;

    App::build()
        .add_resource(GameState {
            screen_dims: screen_dims.clone(),
            mouse_pos: Vec2::new(0., 0.),
            hovered_entity: None,
            selected_entity0: None,
            selected_entity: None,
            material_handle: None,
            stroked_material_handle: None,
            image_dims,
            image_path,
            image_url: image_md.url,
            image_credits: format!("{} by {}", image_md.title, image_md.author),
        })
        .add_resource(ClearColor(Color::rgb(0.05, 0.05, 0.02)))
        .add_resource(WindowDescriptor {
            title: "focus point".to_string(),
            width: screen_dims.x,
            height: screen_dims.y,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(game_setup_system.system())
        .add_system(mouse_handling_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
