use bevy_dev::quad_mesh::build_quad_uvs;

use bevy::{
    prelude::*,
    render::{
        mesh::{Mesh},
    }
};

struct Rotator;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    //let dt = time.delta_seconds();
    let t = time.seconds_since_startup() as f32;
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(1. * t);
    }
}

struct Translator {
    tr: Vec3,
}

fn si(n:f32, scale:f32, period:f32, shift:f32) -> f32 {
    ((n * period) % 3.1415927).sin() * scale + shift
}

fn translator_system(time: Res<Time>, mut query: Query<(&Translator, &mut Transform), With<Translator>>) {
    //let dt = time.delta_seconds();
    let t = time.seconds_since_startup() as f32;
    for (i, (translator, mut transform)) in query.iter_mut().enumerate() {
        let tr = translator.tr;
        let dti1 = (i as f32) * 0.025;
        let dti2 = 1.0 + (i as f32) * 0.025;
        transform.translation = Vec3::new(
            tr[0] + si(
                t + dti1, // + dti1,
                100.,// * dti2,
                2.,
                -50.
            ),
            tr[1],
            tr[2]
        );
    }
}

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
        .add_system(translator_system.system())
        //.add_system(rotator_system.system())
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
    let img_w = 2048.;
    let img_h = 1135.;
    let s = 0.33;
    let w = s;
    let h = s;
    

    commands.spawn(Camera2dBundle::default());

    const W:usize = 4;
    const H:usize = 3;

    let du = 1.0 / W as f32;
    let dv = 1.0 / H as f32;
    for ih in 0..H {
        for iw in 0..W {
            let u0 = (iw as f32) * 1.0 / (W as f32);
            let v0 = (ih as f32) * 1.0 / (H as f32);
            let translation = Vec3::new(
                //0.,
                //0.,
                (-0.5 + ((iw as f32) + 0.5) * du) * w * img_w,
                (0.5 - ((ih as f32) + 0.5) * dv) * h * img_h,
                0.
            );
            commands.spawn(SpriteBundle {
                mesh: meshes.add(build_quad_uvs(w*du, h*dv, u0, u0 + du, v0, v0 + dv)),
                //mesh: meshes.add(build_quad_uvs(w, h, 0., 1., 0., 1.)),
                material: materials.add(img_tex.clone().into()),
                //sprite: Sprite::new(Vec2::new(s*du*ar, s*dv)),
                //sprite: Sprite::new(Vec2::new(1., 1.)),
                transform: Transform::from_translation(translation.clone()),
                ..Default::default()
            })
            //.with(Rotator);
            .with(Translator { tr: translation });
        }
    }
}
