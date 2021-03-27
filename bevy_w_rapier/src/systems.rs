//use super::cylinder::generate_cylinder;
use super::resources::*;
use bevy::prelude::*;
use bevy_rapier3d::physics::{JointBuilderComponent, RigidBodyHandleComponent};
use bevy_rapier3d::rapier::dynamics::{BallJoint, RevoluteJoint, RigidBodyBuilder, RigidBodySet};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::rapier::na::{Point3, Vector3};

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
    .with(Rotates); */

    //return;

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

    /* // Static rigid-body with a cuboid shape.
    let rigid_body1 = RigidBodyBuilder::new_static();
    let collider1 = ColliderBuilder::cuboid(10.0, 1.0, 10.0);
    // Keep the entity identifier.
    let entity1 = commands
        .spawn((rigid_body1, collider1))
        .current_entity()
        .unwrap();

    // Dynamic rigid-body with ball shape.
    let rigid_body2 = RigidBodyBuilder::new_dynamic().translation(0.0, 3.0, 0.0);
    let collider2 = ColliderBuilder::ball(0.5);
    // Keep the entity identifier.
    let entity2 = commands
        .spawn((rigid_body2, collider2))
        .current_entity()
        .unwrap(); */

    // Create the joint.
    //let joint_params = BallJoint::new(Point3::origin(), Point3::new(0.0, -3.0, 0.0));
    //let joint_builder_component = JointBuilderComponent::new(joint_params, entity1, entity2);
    //commands.spawn((joint_builder_component,));

    //return;

    let box_ent: Entity;
    {
        // box
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(0., 3.5, 0.);
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
        commands
            .spawn((rigid_body, collider, pbr))
            .with(Player(2000.0));

        box_ent = commands.current_entity().unwrap();
    }

    for zs in vec![-1., 1.] {
        for xs in vec![-1., 1.] {
            // 4 wheels
            let r = 1.;
            let px = 3. * xs;
            let py = 1.;
            let pz = 3.5 * zs;
            let rigid_body = RigidBodyBuilder::new_dynamic().translation(px, py, pz);
            let collider = ColliderBuilder::ball(r);
            commands.spawn((rigid_body, collider));

            let wheel_ent = commands.current_entity().unwrap();

            let o = Point3::origin();
            let z = Vector3::z_axis();
            let p_ = Point3::new(-px, -py - 1.0, -pz);

            let rev_joint = RevoluteJoint::new(p_, z, o, z);
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

/* fn print_events(events: Res<EventQueue>) {
    while let Ok(_intersection_event) = events.intersection_events.pop() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    while let Ok(_contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
} */
