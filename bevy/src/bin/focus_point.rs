use bevy::{prelude::*, window::WindowMode};
use bevy_dev::{
    arguments::*, game_logic::*, image_metadatas::select_image, resources::*, systems::*,
};

fn main() {
    let args = parse_arguments();
    println!("{:?}", &args);

    let screen_dims = args.screen_dims;
    let image_md = select_image(args.image_index);
    let image_path = format!("textures/images/{}.jpg", image_md.file_name);
    let scale = image_contain(screen_dims, image_md.dims);
    let image_dims = image_md.dims * scale;
    let window_mode = if args.full_screen {
        WindowMode::Fullscreen { use_size: true }
    } else {
        WindowMode::Windowed
    };
    App::build()
        .add_resource(GameState {
            num_pieces: args.num_pieces,
            screen_dims: screen_dims.clone(),
            mouse_pos: Vec2::new(0., 0.),
            hovered_entity: None,
            selected_entity0: None,
            selected_entity: None,
            material_handle: None,
            image_handle: None,
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
            resizable: true,
            vsync: true,
            mode: window_mode,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_event::<MyEvent>()
        .init_resource::<EventTriggerState>()
        .add_startup_system(game_setup_system.system())
        .add_system(animate_system.system())
        .add_system(event_trigger_system.system())
        .add_system(is_puzzle_complete_system.system())
        .add_system(mouse_handling_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
