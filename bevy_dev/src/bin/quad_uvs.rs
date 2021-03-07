use bevy_dev::quad_mesh::build_quad_uvs;

use bevy::{
    prelude::*,
    render::{
        mesh::{Mesh},
    }
};

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)))
        .add_resource(WindowDescriptor {
            title: "quad_uvs".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let img_tex = asset_server.load("textures/images/23364494180_b99e33a74d_k.jpg");
    let s = 0.01_f32;
    let w = 2048. * s;
    let h = 1135. * s;
    let mesh = build_quad_uvs(20., 20., 0., 1., 0., 1.);

    commands
        .spawn(Camera2dBundle::default())

        .spawn(SpriteBundle {
            mesh: meshes.add(mesh),
            material: materials.add(img_tex.clone().into()),
            sprite: Sprite::new(Vec2::new(w, h)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        });
}
