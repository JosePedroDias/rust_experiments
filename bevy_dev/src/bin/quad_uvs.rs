use bevy_dev::quad_mesh::build_quad_uvs;

use bevy::{prelude::*, render::mesh::Mesh};

// RESOURCES

struct GameState {
    tile_index: Option<usize>,
    image_dims: Vec2,
    image_path: String,
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
    mut game_state: ResMut<GameState>,
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
        //println!("World coords: {:.0} x {:.0}", pw.x, pw.y);

        let mut nearest_ti = None;
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
                nearest_ti = Some(ti);
            }
        }

        if nearest_ti.is_some() && nearest_dist < 11500. {
            if nearest_ti != game_state.tile_index {
                println!("tile #{:?} {:.0}", nearest_ti, nearest_dist);
                game_state.tile_index = nearest_ti;
            }
        } else if game_state.tile_index.is_some() {
            println!("tile #NONE {:.0}", nearest_dist);
            game_state.tile_index = None;
        }
    }
}

fn mouse_click_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_tile: Query<(Entity, &TileData), With<TileData>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if game_state.tile_index.is_some() {
            let ti = game_state.tile_index.unwrap();
            //println!("trying to kill #{}", ti);

            for (entity, td) in q_tile.iter() {
                if td.index == ti {
                    println!("{:?}", entity);
                    println!("KILLING #{}!", ti);
                    commands.despawn(entity);

                    // TODO temporary spawn
                    let img_tex = asset_server.load(&game_state.image_path[..]);
                    let (w, h) = game_state.image_dims.into();
                    const W: usize = 4;
                    const H: usize = 3;
                    let du = 1.0 / W as f32;
                    let dv = 1.0 / H as f32;
                    let iw = 0.5;
                    let ih = 1.2;

                    let u0 = (iw as f32) * 1.0 / (W as f32);
                    let v0 = (ih as f32) * 1.0 / (H as f32);
                    let center = Vec3::new(
                        (-0.5 + ((iw as f32) + 0.5) * du) * w,
                        (0.5 - ((ih as f32) + 0.5) * dv) * h,
                        0.,
                    );

                    commands
                        .spawn(SpriteBundle {
                            mesh: meshes.add(build_quad_uvs(
                                w * du,
                                h * dv,
                                u0,
                                u0 + du,
                                v0,
                                v0 + dv,
                            )),
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
                }
            }

            game_state.tile_index = None;
        }
    }
}

// SETUP (STARTUP SYSTEM)

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (w, h) = game_state.image_dims.into();
    let img_tex = asset_server.load(&game_state.image_path[..]);

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
    let image_path = "textures/images/23364494180_b99e33a74d_k.jpg";
    let mut image_dims = Vec2::new(2048., 1135.);
    image_dims *= 0.33;

    App::build()
        .add_resource(GameState {
            tile_index: None,
            image_dims,
            image_path: String::from(image_path),
        })
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
