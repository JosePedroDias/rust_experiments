//use super::cylinder::generate_cylinder;
use super::resources::*;
use bevy::prelude::*;
use bevy_rapier3d::physics::RigidBodyHandleComponent;
use bevy_rapier3d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::na::Vector3;

pub fn setup_scene(
    commands: &mut Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let zoom = 0.5; // simplest_car 1.0, alpine2 0.1, no geo 0.5
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000., 100., 2000.)),
        ..Default::default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-20. * zoom, 20. * zoom, 50. * zoom), // from
                Vec3::new(0., 0., 0.),                          // target
                Vec3::new(0., 1., 0.),                          // up
            )),
            ..Default::default()
        })
        .with(Camera);

    // empty, #Scene0, #Mesh0/Cube
    /* commands
    .spawn_scene(_asset_server.load("alpine2.gltf"))
    //.spawn_scene(_asset_server.load("simplest_car.glb"))
    .with(Rotates); */

    //return;

    /*
    box:
        semi-sizes: 4 / 2 / 2
        position: 0, 2, 0
    wheels (x4):
        semi-height: 0.25
        radius: 1
        position: 3, 1, 2.5
    revolute joints (x4)
    a way to rotate the wheel joints relative to the main body
    */

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
        // box
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(0., 2., 0.);
        let w = 4.;
        let h = 2.;
        let d = 2.;
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
    /* {
        // wheel
        let h2 = 0.25;
        let r = 1.;
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(3., 10., 2.5);
        //let collider = ColliderBuilder::cylinder(h2, r); // along the YY axis
        let collider = ColliderBuilder::ball(r);
        let color = Color::rgb(1., 1., 0.);
        //let mesh = generate_cylinder(h2, r, 16);
        //let mesh = Mesh::from(shape::Box::new(r, r, r));
        let mesh = Mesh::from(shape::Cube { size: r });
        /* let mesh = Mesh::from(shape::Icosphere {
            subdivisions: 2,
            radius: r,
        }); */
        let pbr = PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(color.into()),
            ..Default::default()
        };

        commands.spawn((rigid_body, collider, pbr));
        //.with(Player(200.0));
    } */
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

/* pub fn directing_system(
    keyboard_input: Res<Input<KeyCode>>,
    q_player: Query<(&Player, &RigidBodyHandleComponent)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let x_axis =
        -(keyboard_input.pressed(KeyCode::NU) as i8) + (keyboard_input.pressed(KeyCode::D) as i8);
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
} */

pub fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
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
