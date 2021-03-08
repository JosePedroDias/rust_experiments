use bevy_dev::quad_mesh::build_quad_uvs;

use bevy::{prelude::*, render::mesh::Mesh};

const W: usize = 4;
const H: usize = 3;

// RESOURCES

struct GameState {
    hovered_entity: Option<Entity>,
    selected_entity: Option<Entity>,
    image_dims: Vec2,
    image_path: String,
}

// WITHS

struct MainCamera;

#[derive(Clone)]
struct TileData {
    index: usize,
    center: Vec3,
    mesh_params: (f32, f32, f32, f32, f32, f32),
}

// SYSTEMS

fn cursor_system(
    ev_cursor: Res<Events<CursorMoved>>,
    mut evr_cursor: Local<EventReader<CursorMoved>>,
    wnds: Res<Windows>,
    mut game_state: ResMut<GameState>,
    q_camera: Query<&Transform, With<MainCamera>>,
    q_tile: Query<(Entity, &TileData), With<TileData>>,
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

        let mut nearest_ent = None;
        let mut nearest_dist: f32 = std::f32::MAX;

        for (entity, td) in q_tile.iter() {
            let center = td.center;

            let dx = center.x - pw.x;
            let dy = center.y - pw.y;
            let dist = dx * dx + dy * dy;

            if dist < nearest_dist {
                nearest_dist = dist;
                nearest_ent = Some(entity);
            }
        }

        if nearest_ent.is_some() && nearest_dist < 11500. {
            if nearest_ent != game_state.hovered_entity {
                println!("hover tile #{:?}   {:.0}", nearest_ent, nearest_dist);
                game_state.hovered_entity = nearest_ent;
            }
        } else if game_state.hovered_entity.is_some() {
            println!("hover tile #NONE {:.0}", nearest_dist);
            game_state.hovered_entity = None;
        }
    }
}

fn mouse_click_system(
    commands: &mut Commands,
    //asset_server: Res<AssetServer>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &TileData), With<TileData>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if game_state.hovered_entity.is_none() {
            return;
        }

        if game_state.selected_entity.is_none() {
            game_state.selected_entity = game_state.hovered_entity;
            return;
        }
        let ent1 = game_state.selected_entity.unwrap();
        let ent2 = game_state.hovered_entity.unwrap();

        if ent1 == ent2 {
            return;
        }

        println!("swapping #{:?} <-> #{:?}", ent1, ent2);

        let mut td1: Option<TileData> = None;
        let mut td2: Option<TileData> = None;

        for (entity, td) in query.iter() {
            if entity == ent1 {
                td1 = Some(td.clone());
            } else if entity == ent2 {
                td2 = Some(td.clone());
            }
        }
        println!("GOT HERE");

        commands.despawn(ent1);
        commands.despawn(ent2);

        game_state.selected_entity = None;
        game_state.hovered_entity = None;

        /* if game_state.selected_index.is_none() {

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
        } */
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

    let du = 1.0 / W as f32;
    let dv = 1.0 / H as f32;
    let mut ti: usize = 0;
    for ih in 0..H {
        for iw in 0..W {
            let tw = w * du;
            let th = h * dv;
            let u0 = (iw as f32) * 1.0 / (W as f32);
            let v0 = (ih as f32) * 1.0 / (H as f32);
            let u1 = u0 + du;
            let v1 = v0 + dv;
            let center = Vec3::new(
                (-0.5 + ((iw as f32) + 0.5) * du) * w,
                (0.5 - ((ih as f32) + 0.5) * dv) * h,
                0.,
            );
            commands
                .spawn(SpriteBundle {
                    mesh: meshes.add(build_quad_uvs(tw, th, u0, u1, v0, v1)),
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
                    mesh_params: (tw, th, u0, u1, v0, v1),
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
            hovered_entity: None,
            selected_entity: None,
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
