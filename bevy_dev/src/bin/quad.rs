mod quad_mesh;

use quad_mesh::*;

use bevy::{
    prelude::*,
    render::{
        mesh::{Mesh},
    },
    window::WindowResized,
};

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)))
        .add_resource(WindowDescriptor {
            title: "quad".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(window_resize.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn window_resize(resize_event: Res<Events<WindowResized>>, mut window: ResMut<WindowDescriptor>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("resize: {} x {}", e.width, e.height);
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let uv_tex = asset_server.load("textures/uv-test.png");

    commands
        .spawn(Camera2dBundle::default())

        .spawn(SpriteBundle {
            mesh: meshes.add(quad()),
            material: materials.add(uv_tex.clone().into()),
            sprite: Sprite::new(Vec2::new(20., 20.)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        });
}
