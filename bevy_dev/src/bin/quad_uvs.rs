use bevy_dev::quad_mesh::build_quad_uvs;

use bevy::{prelude::*, render::mesh::Mesh};

// RESOURCES

struct Selection {
    tile_index: Option<usize>,
}

// WITHS

struct MainCamera;

struct TileData {
    index: usize,
    center: Vec3,
}

// SYSTEMS

fn cursor_system(
    ev_cursor: Res<Events<CursorMoved>>,
    mut evr_cursor: Local<EventReader<CursorMoved>>,
    wnds: Res<Windows>,
    mut selection: ResMut<Selection>,
    q_camera: Query<&Transform, With<MainCamera>>,
    q_tile: Query<&TileData, With<TileData>>,
) {
    let camera_transform = q_camera.iter().next().unwrap();
    let mut pos_wld: Option<Vec4> = None;

    for ev in evr_cursor.iter(&ev_cursor) {
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        //println!("size: {} x {}", size.x, size.y);
        let p = ev.position - size / 2.0;
        pos_wld = Some(camera_transform.compute_matrix() * p.extend(0.0).extend(1.0));
    }

    if pos_wld.is_some() {
        let pw = pos_wld.unwrap();
        //eprintln!("World coords: {:.0} x {:.0}", pw.x, pw.y);

        let mut nearest_ti = 0;
        let mut nearest_dist: f32 = std::f32::MAX;

        for td in q_tile.iter() {
            let ti = td.index;
            let center = td.center;
            //println!("{:?} {:?}", ti, tr);

            let dx = center.x - pw.x;
            let dy = center.y - pw.y;
            let dist = dx * dx + dy * dy;

            if dist < nearest_dist {
                nearest_dist = dist;
                nearest_ti = ti;
            }
        }

        println!("tile #{:?}", nearest_ti);
        selection.tile_index = Some(nearest_ti);
    }
}

fn mouse_click_system(mouse_button_input: Res<Input<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        println!("left mouse currently pressed");
    }
}

// SETUP (STARTUP SYSTEM)

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

    commands.spawn(Camera2dBundle::default()).with(MainCamera);

    const W: usize = 4;
    const H: usize = 3;

    let du = 1.0 / W as f32;
    let dv = 1.0 / H as f32;
    let mut ti: usize = 0;
    for ih in 0..H {
        for iw in 0..W {
            let u0 = (iw as f32) * 1.0 / (W as f32);
            let v0 = (ih as f32) * 1.0 / (H as f32);
            let center = Vec3::new(
                (-0.5 + ((iw as f32) + 0.5) * du) * w,
                (0.5 - ((ih as f32) + 0.5) * dv) * h,
                0.,
            );
            commands
                .spawn(SpriteBundle {
                    mesh: meshes.add(build_quad_uvs(w * du, h * dv, u0, u0 + du, v0, v0 + dv)),
                    material: materials.add(img_tex.clone().into()),
                    sprite: Sprite {
                        size: Vec2::new(1., 1.),
                        resize_mode: SpriteResizeMode::Manual,
                    },
                    transform: Transform::from_translation(center.clone()),
                    ..Default::default()
                })
                .with(TileData {
                    index: ti,
                    center: center,
                });
            ti += 1;
        }
    }
}

// MAIN

fn main() {
    App::build()
        .add_resource(Selection { tile_index: None })
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
        .add_system(cursor_system.system())
        .add_system(mouse_click_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
