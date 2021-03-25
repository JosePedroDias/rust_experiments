use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_w_rapier::systems::*;

use bevy_rapier3d::render::RapierRenderPlugin;

//use bevy_w_rapier::cylinder::generate_cylinder;
//use bevy_w_rapier::sys;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(create_collider_renders_system.system())
        .add_system(move_system.system())
        //.add_system(print_events.system())
        .run();
}
