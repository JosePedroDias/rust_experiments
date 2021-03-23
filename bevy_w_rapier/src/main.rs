use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_rapier3d::physics::{
    ColliderHandleComponent,
    RapierConfiguration,
    RapierPhysicsPlugin,
    RigidBodyHandleComponent, //EventQueue
};
use bevy_rapier3d::rapier::geometry::{ColliderBuilder, ColliderSet};
//use bevy_rapier3d::rapier::dynamics
use bevy_rapier3d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier3d::render::RapierRenderPlugin;
//use bevy_rapier3d::dynamics::{RigidBodyBuilder, RigidBodySet};
//use bevy_rapier3d::geometry::ColliderSet;
//use bevy_rapier3d::geometry::BroadPhase;
//use bevy_rapier3d::geometry::ColliderBuilder;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        //.add_system(print_events.system())
        .add_system(create_collider_renders_system.system())
        .run();
}

fn setup_graphics(commands: &mut Commands) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-30.0, 30.0, 100.0),
                Vec3::new(0.0, 10.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

fn setup_physics(commands: &mut Commands) {
    // ground
    let ground_size = 200.1;
    let ground_height = 0.1;

    let rigid_body = RigidBodyBuilder::new_static().translation(0.0, -ground_height, 0.0);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_size);
    commands.spawn((rigid_body, collider));

    // cubes
    let num = 3;
    let rad = 1.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..2 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;

                // Build the rigid body.
                let rigid_body = RigidBodyBuilder::new_dynamic().translation(x, y, z);
                let collider = ColliderBuilder::cuboid(rad, rad, rad).density(1.0);
                commands.spawn((rigid_body, collider));
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}

pub fn create_collider_renders_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //configuration: Res<RapierConfiguration>,
    bodies: Res<RigidBodySet>,
    colliders: ResMut<ColliderSet>,
    query: Query<(Entity, &ColliderHandleComponent)>,
) {
    let ground_color = Color::rgb(
        0xF3 as f32 / 255.0,
        0xD9 as f32 / 255.0,
        0xB1 as f32 / 255.0,
    );

    for (entity, collider) in &mut query.iter() {
        if let Some(collider) = colliders.get(collider.handle()) {
            if let Some(body) = bodies.get(collider.parent()) {
                let color = if body.is_static() {
                    ground_color
                } else {
                    ground_color
                };
                let shape = collider.shape();

                let mesh = Mesh::from(shape::Cube { size: 2.0 });

                /* let mesh = match shape.shape_type() {
                    ShapeType::Cuboid => Mesh::from(shape::Cube { size: 2.0 }),
                    ShapeType::Ball => Mesh::from(shape::Icosphere {
                        subdivisions: 2,
                        radius: 1.0,
                    }),
                }; */

                /*let scale = match shape.shape_type() {
                    ShapeType::Cuboid => {
                        let c = shape.as_cuboid().unwrap();
                        Vec3::from_slice_unaligned(c.half_extents.as_slice())
                    }
                    ShapeType::Ball => {
                        let b = shape.as_ball().unwrap();
                        Vec3::new(b.radius, b.radius, b.radius)
                    }
                } * configuration.scale;*/

                //let b = shape.as_ball().unwrap();
                //let scale = Vec3::new(b.radius, b.radius, b.radius);

                let transform = Transform::from_scale(Vec3::one());
                /*crate::physics::sync_transform(
                    collider.position_wrt_parent(),
                    configuration.scale,
                    &mut transform,
                );*/

                let pbr = PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(color.into()),
                    transform,
                    ..Default::default()
                };

                commands.insert(entity, pbr);
            }
        }
    }
}

/* fn print_events(events: Res<EventQueue>) {
    while let Ok(intersection_event) = events.intersection_events.pop() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
} */

/* fn sys1(q: Query<&RigidBodyHandleComponent>) {
    // RigidBody
    // Collider
    // Joint
    for rb in q.iter() {
        let x = &*rb;
        println!("{:?}", x);
    }
} */

/* fn my_system(mut broad_phase: ResMut<BroadPhase>, rigid_bodies: Res<RigidBodySet>) {
    //println!("{:?}", rigid_bodies.bodies.len());
    //for body in rigid_bodies./ {}
    if let Some(rb) = rigid_bodies.get_mut(rigid_body_component.handle()) {
        rb.set_linvel(move_delta * player.0, true);
    }
} */
