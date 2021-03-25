use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_rapier3d::physics::{
    ColliderHandleComponent,
    RapierPhysicsPlugin,
    RigidBodyHandleComponent,
    //EventQueue,
    //RapierConfiguration
};
use bevy_rapier3d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier3d::rapier::geometry::{
    ColliderBuilder,
    ColliderSet,
    //ShapeType,
    //BroadPhase,
};
use bevy_rapier3d::rapier::na::Vector3;
use bevy_rapier3d::render::RapierRenderPlugin;

use bevy::{
    render::mesh::{Indices, Mesh},
    render::pipeline::PrimitiveTopology,
};

//use rapier3d::math::Rotation;

const PI2: f32 = std::f32::consts::PI * 2.0;

// Resources
struct Player(f32);
pub enum MyShape {
    Box(f32, f32, f32),
    Sphere(f32),
    Cylinder(f32, f32),
}

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

fn setup_graphics(commands: &mut Commands) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-20.0, 20.0, 50.0),
                Vec3::new(0.0, 10.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

fn generate_cylinder(half_height: f32, radius: f32, steps: usize) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let n_vertices = 2 + steps * 4;
    let n_triangles = (steps * 4) as u32;

    // TODO UVS

    let h2 = half_height;
    /*
    vertices:
        ctr -y
        ctr +y
        bottom circ
        top circ
        sides (2 circs)
    triangles:
        bottom
        top
        side (2 tris at a time)
     */

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity((n_triangles as usize) * 3);

    positions.push([0., -h2, 0.]);
    normals.push([0., -1., 0.]);
    uvs.push([0., 0.]);

    positions.push([0., h2, 0.]);
    normals.push([0., 1., 0.]);
    uvs.push([0., 0.]);

    for nth in 0..2 {
        let y = if nth == 0 { -h2 } else { h2 };
        let ny = if nth == 0 { -1. } else { 1. };
        for i in 0..steps {
            let angle = (i as f32) / (steps as f32) * PI2;
            let c = angle.cos();
            let s = angle.sin();
            let x = radius * c;
            let z = radius * s;
            positions.push([x, y, z]);
            normals.push([0., ny, 0.]);
            uvs.push([0., 0.]);
        }
    }

    for nth in 0..2 {
        let y = if nth == 0 { -h2 } else { h2 };
        for i in 0..steps {
            let angle = (i as f32) / (steps as f32) * PI2;
            let c = angle.cos();
            let s = angle.sin();
            let x = radius * c;
            let z = radius * s;
            positions.push([x, y, z]);
            normals.push([c, 0., s]);
            uvs.push([0., 0.]);
        }
    }

    let s0 = 2 as u32;
    let s1 = 2 + steps as u32;
    let s2 = 2 + (steps * 2) as u32;
    let s3 = 2 + (steps * 3) as u32;

    // bottom and top
    for i in 0..n_triangles {
        indices.push(i % n_triangles + s0);
        indices.push((i + 1) % n_triangles + s0);
        indices.push(0);

        indices.push(i % n_triangles + s1);
        indices.push((i + 1) % n_triangles + s1);
        indices.push(1);
    }

    // sides
    for i in 0..n_triangles {
        indices.push(i % n_triangles + s3);
        indices.push((i + 1) % n_triangles + s2);
        indices.push(i % n_triangles + s2);

        indices.push(i % n_triangles + s3);
        indices.push((i + 1) % n_triangles + s3);
        indices.push((i + 1) % n_triangles + s2);
    }

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

fn setup_physics(commands: &mut Commands) {
    {
        // ground
        let ground_size = 200.0;
        let ground_height = 0.1;
        let rigid_body = RigidBodyBuilder::new_static().translation(0.0, -ground_height, 0.0);
        let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_size);
        commands.spawn((rigid_body, collider)).with(MyShape::Box(
            ground_size,
            ground_height,
            ground_size,
        ));
    }
    {
        // sphere
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(0.0, 30.0, 0.0);
        let collider = ColliderBuilder::ball(1.);
        commands
            .spawn((rigid_body, collider))
            .with(MyShape::Sphere(1.3))
            .with(Player(200.0));
    }
    {
        let rigid_body = RigidBodyBuilder::new_static().translation(-20.0, 1.0, -5.0);
        let collider = ColliderBuilder::cuboid(1.0, 2.0, 3.0);
        commands
            .spawn((rigid_body, collider))
            .with(MyShape::Box(1.0, 2.0, 3.0));
    }

    {
        // cubes
        let num = 3;
        let num2 = 2;
        let num3 = 1;
        let rad = 1.5;

        let shift = rad * 2.0 + rad;
        let centerx = shift * (num / 2) as f32;
        let centery = shift / 2.0;
        let centerz = shift * (num / 2) as f32;

        let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

        for j in 0usize..num2 {
            for i in 0..num {
                for k in 0usize..num3 {
                    let x = i as f32 * shift - centerx + offset;
                    let y = j as f32 * shift + centery + 3.0;
                    let z = k as f32 * shift - centerz + offset;

                    let rigid_body = RigidBodyBuilder::new_dynamic().translation(x, y, z);
                    let collider = ColliderBuilder::cuboid(rad, rad, rad).density(0.5);
                    commands.spawn((rigid_body, collider)).with(MyShape::Box(
                        rad * 2.0,
                        rad * 2.0,
                        rad * 2.0,
                    ));
                }
            }

            offset -= 0.05 * rad * (num as f32 - 1.0);
        }
    }

    {
        // wheel
        let r = 2.0;
        let h2 = 0.66;
        let axis_angle = Vector3::x() * -std::f32::consts::FRAC_PI_2;
        // Point and vector being transformed in the tests.

        let rigid_body = RigidBodyBuilder::new_dynamic()
            .rotation(axis_angle)
            .translation(0.0, 50.0, 0.0);
        let collider = ColliderBuilder::cylinder(h2, r);
        commands
            .spawn((rigid_body, collider))
            .with(MyShape::Cylinder(h2, r));
    }
}

pub fn create_collider_renders_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    bodies: Res<RigidBodySet>,
    colliders: ResMut<ColliderSet>,
    query: Query<(Entity, &ColliderHandleComponent, &MyShape)>,
) {
    let color1 = Color::rgb(
        0xFF as f32 / 255.0,
        0x00 as f32 / 255.0,
        0x00 as f32 / 255.0,
    );

    let color2 = Color::rgb(
        0x00 as f32 / 255.0,
        0xFF as f32 / 255.0,
        0x00 as f32 / 255.0,
    );

    for (entity, collider, ms) in query.iter() {
        //if let Ok((health, mut transform)) = query.get_mut(entity) {
        if let Some(collider) = colliders.get(collider.handle()) {
            if let Some(body) = bodies.get(collider.parent()) {
                let color = if body.is_static() { color1 } else { color2 };
                //let shape = collider.shape();

                let mesh = match ms {
                    MyShape::Box(a, b, c) => Mesh::from(shape::Box::new(*a, *b, *c)),
                    MyShape::Sphere(r) => Mesh::from(shape::Icosphere {
                        subdivisions: 2,
                        radius: *r,
                    }),
                    MyShape::Cylinder(a, b) => generate_cylinder(*a, *b, 16),
                };
                let transform = Transform::from_scale(Vec3::one());
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
    while let Ok(_intersection_event) = events.intersection_events.pop() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    while let Ok(_contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
} */

fn move_system(
    keyboard_input: Res<Input<KeyCode>>,
    q_player: Query<(&Player, &RigidBodyHandleComponent)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let x_axis =
        -(keyboard_input.pressed(KeyCode::A) as i8) + (keyboard_input.pressed(KeyCode::D) as i8);
    let z_axis =
        -(keyboard_input.pressed(KeyCode::W) as i8) + (keyboard_input.pressed(KeyCode::S) as i8);
    let move_delta = Vector3::new(x_axis as f32, 0.0, z_axis as f32);
    for (player, rigid_body_component) in q_player.iter() {
        if let Some(rb) = rigid_bodies.get_mut(rigid_body_component.handle()) {
            if move_delta.x != 0.0 || move_delta.z != 0.0 {
                //rb.set_linvel(move_delta * player.0, true);
                rb.apply_force(move_delta * player.0, true);
            }
        }
    }
}
