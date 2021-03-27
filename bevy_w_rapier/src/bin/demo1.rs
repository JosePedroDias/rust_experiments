use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_w_rapier::resources::*;
use bevy_w_rapier::systems::*;

//use bevy_rapier3d::render::RapierRenderPlugin;

fn main() {
    App::build()
        .add_resource(CameraState {
            from: Vec3::new(-20., 20., 50.),
            up: Vec3::new(0., 1., 0.),
        })
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_resource(WindowDescriptor {
            title: "bevy with rapier".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        //.add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_scene.system())
        .add_system(rotator_system.system())
        .add_system(move_system.system())
        .add_system(direct_system.system())
        //.add_system(print_events.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
