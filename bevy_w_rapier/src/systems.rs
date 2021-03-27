use super::components::*;
use super::cylinder::generate_cylinder;
use super::resources::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_rapier3d::physics::{JointBuilderComponent, RigidBodyHandleComponent};
use bevy_rapier3d::rapier::dynamics::{RevoluteJoint, RigidBodyBuilder, RigidBodySet};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::na::{Point3, Vector3};

pub fn setup_scene(
    cam_state: Res<CameraState>,
    commands: &mut Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(40., 10., 20.)),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_matrix(Mat4::face_toward(
            cam_state.from,        // from
            Vec3::new(0., 0., 0.), // target
            cam_state.up,          // up
        )),
        ..Default::default()
    });

    // empty, #Scene0, #Mesh0/Cube
    /* commands
    .spawn_scene(_asset_server.load("alpine2.gltf"))
    .with(Rotates); */

    {
        // ground
        let w = 20.;
        let h = 0.1;
        let d = 20.;
        let rigid_body = RigidBodyBuilder::new_static().translation(0., -h, 0.);
        let collider = ColliderBuilder::cuboid(w, h, d);
        let color = Color::rgb(0.2, 0.2, 0.2);
        let mesh = Mesh::from(shape::Box::new(w * 2., h * 2., d * 2.));
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(color.into()),
                ..Default::default()
            })
            .with(rigid_body)
            .with(collider);
    }

    let box_ent: Entity;
    {
        // box
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(0., 3.5, 0.);
        let w = 4.;
        let h = 2.;
        let d = 2.;
        let collider = ColliderBuilder::cuboid(w, h, d);
        let color = Color::rgb(0.25, 0.25, 1.);
        let mesh = Mesh::from(shape::Box::new(w * 2., h * 2., d * 2.));
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(color.into()),
                ..Default::default()
            })
            .with(rigid_body)
            .with(collider)
            .with(Focus)
            .with(Player(2000.0));

        box_ent = commands.current_entity().unwrap();
    }

    for zs in vec![-1., 1.] {
        for xs in vec![-1., 1.] {
            // 4 wheels
            let h2 = 0.25;
            let r = 1.;
            let px = 3. * xs;
            let py = 1.;
            let pz = 3.5 * zs;
            let rigid_body = RigidBodyBuilder::new_dynamic().translation(px, py, pz);
            let collider = ColliderBuilder::ball(r);

            let color = Color::rgb(1., 0.25, 0.25);
            let mesh = generate_cylinder(h2, r, 16);
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(color.into()),
                    ..Default::default()
                })
                .with(rigid_body)
                .with(collider);

            let wheel_ent = commands.current_entity().unwrap();

            let o = Point3::origin();
            let z = Vector3::z_axis();
            let y = Vector3::y_axis();
            let p_ = Point3::new(-px, -py - 1.0, -pz);

            let rev_joint = RevoluteJoint::new(p_, z, o, y);
            commands.spawn((JointBuilderComponent::new(rev_joint, box_ent, wheel_ent),));
        }
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

pub fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}

pub fn direct_system(
    cam_state: Res<CameraState>,
    q_focus: Query<&Transform, With<Focus>>,
    mut q_cam: Query<&mut Transform, With<Camera>>,
) {
    for focus_trans in q_focus.iter() {
        for mut cam_trans in q_cam.iter_mut() {
            let to: Vec3 = (*focus_trans).translation;
            let mtx = Mat4::face_toward(
                cam_state.from, // from
                to,             // target
                cam_state.up,   // up
            );
            *cam_trans = Transform::from_matrix(mtx);
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
