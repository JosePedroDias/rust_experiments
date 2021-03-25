use super::cylinder::generate_cylinder;
use super::resources::*;
use bevy::prelude::*;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::na::Vector3;

pub fn setup_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(1000., 100., 2000.)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-20., 20., 50.),
                Vec3::new(0., 10., 0.),
                Vec3::new(0., 1., 0.),
            )),
            ..Default::default()
        });

    {
        // ground
        let w = 200.;
        let h = 0.1;
        let d = 200.;
        let rigid_body = RigidBodyBuilder::new_static().translation(0., -h, 0.);
        let collider = ColliderBuilder::cuboid(w, h, d);
        let color = Color::rgb(0., 0., 1.);
        let mesh = Mesh::from(shape::Box::new(w, h, d));
        let pbr = PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(color.into()),
            ..Default::default()
        };
        commands.spawn((rigid_body, collider, pbr));
    }
    {
        // sphere
        let r = 1.0;
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(0., 30., 0.);
        let collider = ColliderBuilder::ball(r);
        let color = Color::rgb(1., 0., 0.);
        let mesh = Mesh::from(shape::Icosphere {
            subdivisions: 2,
            radius: r,
        });
        let pbr = PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(color.into()),
            ..Default::default()
        };

        commands
            .spawn((rigid_body, collider, pbr))
            .with(Player(200.0));
    }
    {
        // wheel
        let h2 = 0.33;
        let r = 1.;
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(0., 35., 0.);
        let collider = ColliderBuilder::cylinder(h2, r);
        let color = Color::rgb(0., 1., 0.);
        let mesh = generate_cylinder(h2, r, 16);
        let pbr = PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(color.into()),
            ..Default::default()
        };

        commands.spawn((rigid_body, collider, pbr));
        //.with(Player(200.0));
    }
}

pub fn move_system(
    keyboard_input: Res<Input<KeyCode>>,
    q_player: Query<(&Player, &RigidBodyHandleComponent)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let x_axis =
        -(keyboard_input.pressed(KeyCode::A) as i8) + (keyboard_input.pressed(KeyCode::D) as i8);
    let z_axis =
        -(keyboard_input.pressed(KeyCode::W) as i8) + (keyboard_input.pressed(KeyCode::S) as i8);
    let move_delta = Vector3::new(x_axis as f32, 0., z_axis as f32);
    for (player, rigid_body_component) in q_player.iter() {
        if let Some(rb) = rigid_bodies.get_mut(rigid_body_component.handle()) {
            if move_delta.x != 0. || move_delta.z != 0. {
                rb.apply_force(move_delta * player.0, true);
            }
        }
    }
}

/* fn print_events(events: Res<EventQueue>) {
    while let Ok(_intersection_event) = events.intersection_events.pop() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    while let Ok(_contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
} */
