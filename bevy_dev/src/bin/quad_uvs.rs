use bevy_dev::quad_mesh::build_quad_uvs;

use bevy::{
    prelude::*,
    render::{
        mesh::{Mesh},
    }
};

struct MainCamera;

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

struct TileIndex {
    index: usize
}

fn si(n:f32, scale:f32, period:f32, shift:f32) -> f32 {
    ((n * period) % 3.1415927).sin() * scale + shift
}

fn translator_system(
    time: Res<Time>,
    mut query: Query<(&Translator, &mut Transform), With<Translator>>
) {
    //let dt = time.delta_seconds();
    let t = time.seconds_since_startup() as f32;
    for (i, (translator, mut transform)) in query.iter_mut().enumerate() {
        let tr = translator.tr;
        let _dti1 = (i as f32) * 0.025;
        let _dti2 = 1.0 + (i as f32) * 0.025;
        transform.translation = Vec3::new(
            tr[0] + si(
                t + _dti1, // + _dti1,
                100.,// * _dti2,
                2.,
                -50.
            ),
            tr[1],
            tr[2]
        );
    }
}

fn my_cursor_system(
    // events to get cursor position
    ev_cursor: Res<Events<CursorMoved>>,
    mut evr_cursor: Local<EventReader<CursorMoved>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
    q_tile_index: Query<(&TileIndex, &Translator), With<TileIndex>>
) {
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = q_camera.iter().next().unwrap();

    let mut pos_wld:Option<Vec4> = None;

    for ev in evr_cursor.iter(&ev_cursor) {
        // get the size of the window that the event is for
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        //println!("size: {} x {}", size.x, size.y);

        // the default orthographic projection is in pixels from the center. just undo the translation
        let p = ev.position - size / 2.0;

        // apply the camera transform
        pos_wld = Some(camera_transform.compute_matrix() * p.extend(0.0).extend(1.0));
    }

    if pos_wld.is_some() {
        let pw = pos_wld.unwrap();
        //eprintln!("World coords: {:.0} x {:.0}", pw.x, pw.y);

        let mut nearest_ti = 0;
        let mut nearest_dist:f32 = std::f32::MAX;

        for (tile_index, translator) in q_tile_index.iter() {
            let ti = tile_index.index;
            let tr = translator.tr;
            //println!("{:?} {:?}", ti, tr);

            let dx = tr.x - pw.x;
            let dy = tr.y - pw.y;
            let dist = dx*dx + dy*dy;

            if dist < nearest_dist {
                nearest_dist = dist;
                nearest_ti = ti;
            }
        }

        println!("tile #{:?}", nearest_ti);
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
        .add_system(rotator_system.system())
        .add_system(my_cursor_system.system())
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
    let w = img_w * s;
    let h = img_h * s;
    

    commands.spawn(Camera2dBundle::default())
        .with(MainCamera);

    const W:usize = 4;
    const H:usize = 3;

    let du = 1.0 / W as f32;
    let dv = 1.0 / H as f32;
    let mut ti:usize = 0;
    for ih in 0..H {
        for iw in 0..W {
            let u0 = (iw as f32) * 1.0 / (W as f32);
            let v0 = (ih as f32) * 1.0 / (H as f32);
            let translation = Vec3::new(
                (-0.5 + ((iw as f32) + 0.5) * du) * w,
                (0.5 - ((ih as f32) + 0.5) * dv) * h,
                0.
            );
            commands.spawn(SpriteBundle {
                mesh: meshes.add(build_quad_uvs(w*du, h*dv, u0, u0 + du, v0, v0 + dv)),
                material: materials.add(img_tex.clone().into()),
                sprite: Sprite {
                    //size: Vec2::new(w * du, h * dv),
                    size: Vec2::new(1., 1.),
                    resize_mode: SpriteResizeMode::Manual
                    //resize_mode: SpriteResizeMode::Automatic
                },
                transform: Transform::from_translation(translation.clone()),
                ..Default::default()
            })
            .with(TileIndex { index: ti })
            //.with(Rotator);
            .with(Translator { tr: translation });
            //println!("{}x{} ... {}", w*du, h*dv, translation[0]);

            ti += 1;
        }
    }
}
