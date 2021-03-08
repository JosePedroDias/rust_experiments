use bevy::{prelude::*, render::mesh::Mesh};
use bevy_dev::{image_metadatas::select_random_image, quad_mesh::build_quad_uvs};
use std::mem;

const W: usize = 10;
const H: usize = 8;

fn image_contain(screen_dims: Vec2, image_dims: Vec2) -> f32 {
    let (w, h) = screen_dims.into();
    let (iw, ih) = image_dims.into();
    let sar = w / h;
    let iar = iw / ih;
    return if iar > sar { w / iw } else { h / ih };
}

fn generate_tile_bundle(
    mesh: Handle<Mesh>,
    mat: Handle<ColorMaterial>,
    center: Vec3,
) -> SpriteBundle {
    SpriteBundle {
        mesh: mesh,
        material: mat,
        sprite: Sprite {
            size: Vec2::new(1., 1.),
            resize_mode: SpriteResizeMode::Manual,
        },
        transform: Transform::from_translation(center),
        ..Default::default()
    }
}

// RESOURCES

struct GameState {
    hovered_entity: Option<Entity>,
    selected_entity: Option<Entity>,
    image_dims: Vec2,
    image_path: String,
    material_handle: Option<Handle<ColorMaterial>>,
}

// WITHS

struct MainCamera;

#[derive(Clone)]
struct TileData {
    index: usize,
    center: Vec3,
    dims: Vec2,
    uvs: (f32, f32, f32, f32),
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

        let mut hovered_ent = None;
        let mut hovered_index = None;

        for (entity, td) in q_tile.iter() {
            let center = td.center;
            let dims = td.dims;

            let x0 = center.x - dims.x * 0.5;
            let x1 = center.x + dims.x * 0.5;
            let y0 = center.y - dims.y * 0.5;
            let y1 = center.y + dims.y * 0.5;

            if pw.x > x0 && pw.x < x1 && pw.y > y0 && pw.y < y1 {
                hovered_ent = Some(entity);
                hovered_index = Some(td.index);
                break;
            }
        }

        let was_empty = game_state.hovered_entity.is_none();
        let is_empty = hovered_ent.is_none();
        let was_set = !was_empty;
        let is_set = !is_empty;

        if was_empty && is_empty {
            //println!("hover: STILL EMPTY...");
        } else if was_empty && is_set {
            println!(
                "hover: JUST GOT SOMETHING: #{} ({:?})",
                hovered_index.unwrap(),
                hovered_ent.unwrap()
            );
            game_state.hovered_entity = hovered_ent;
        } else if was_set && is_set {
            if game_state.hovered_entity.unwrap() == hovered_ent.unwrap() {
                //println!("hover: STILL THE SAME...");
            } else {
                println!(
                    "hover: JUST GOT SOMETHING ELSE: #{} ({:?})",
                    hovered_index.unwrap(),
                    hovered_ent.unwrap()
                );
                game_state.hovered_entity = hovered_ent;
            }
        } else if was_set && is_empty {
            println!("hover: JUT GOT EMPTY");
        } else {
            println!("WTF!");
        }
    }
}

fn mouse_click_system(
    commands: &mut Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &TileData), With<TileData>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if game_state.hovered_entity.is_none() {
            //println!("click 0 -> NOTHING HOVERED!");
            return;
        }

        if game_state.selected_entity.is_none() {
            game_state.selected_entity = game_state.hovered_entity;
            game_state.hovered_entity = None;
            //println!("click 1 -> PROMOTING HOVER TO SELECTION");
            return;
        }
        let ent1 = game_state.selected_entity.unwrap();
        let ent2 = game_state.hovered_entity.unwrap();

        game_state.selected_entity = None;
        game_state.hovered_entity = None;

        if ent1 == ent2 {
            //println!("clock 2 -> SAME TILES!");
            return;
        }

        let mut td1: Option<TileData> = None;
        let mut td2: Option<TileData> = None;

        for (entity, td) in query.iter() {
            if entity == ent1 {
                td1 = Some(td.clone());
            } else if entity == ent2 {
                td2 = Some(td.clone());
            }
        }

        commands.despawn(ent1);
        commands.despawn(ent2);

        let mut td1 = td1.unwrap();
        let mut td2 = td2.unwrap();

        println!(
            "swapping #{} <-> #{} ({:?} {:?})",
            td1.index, td2.index, ent1, ent2
        );

        mem::swap(&mut td1.uvs.0, &mut td2.uvs.0);
        mem::swap(&mut td1.uvs.1, &mut td2.uvs.1);
        mem::swap(&mut td1.uvs.2, &mut td2.uvs.2);
        mem::swap(&mut td1.uvs.3, &mut td2.uvs.3);

        let tds = [td1, td2];
        for td in tds.iter() {
            let mat = game_state.material_handle.as_ref().unwrap();
            let mat = (*mat).clone();
            let dims = td.dims.clone();
            let uvs = td.uvs.clone();
            let mesh = meshes.add(build_quad_uvs(dims, uvs));
            commands
                .spawn(generate_tile_bundle(mesh, mat, td.center))
                .with(td.clone());
        }
    }
}

// SETUP (STARTUP SYSTEM)

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (w, h) = game_state.image_dims.into();
    let img_tex = asset_server.load(&game_state.image_path[..]);
    let mat = materials.add(img_tex.clone().into());
    game_state.material_handle = Some(mat.clone());

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
            let dims = Vec2::new(tw, th);
            let uvs = (u0, u1, v0, v1);
            let mesh = meshes.add(build_quad_uvs(dims, uvs));
            let td = TileData {
                center,
                index: ti,
                dims,
                uvs,
            };
            commands
                .spawn(generate_tile_bundle(mesh, mat.clone(), center))
                .with(td);
            ti += 1;
        }
    }
}

// MAIN

fn main() {
    let screen_dims: Vec2 = Vec2::new(1024., 768.);
    static BEFORE_UPDATE: &str = "BEFORE_UPDATE";

    let image_md = select_random_image();
    //println!("{:?}", image_md);
    let image_path = format!("textures/images/{}.jpg", image_md.file_name);
    let scale = image_contain(screen_dims, image_md.dims);
    let image_dims = image_md.dims * scale;

    App::build()
        //.add_resource(DefaultTaskPoolOptions::with_num_threads(1)) // just for debugging if systems are working well
        .add_resource(GameState {
            hovered_entity: None,
            selected_entity: None,
            material_handle: None,
            image_dims,
            image_path,
        })
        .add_resource(ClearColor(Color::rgb(0.05, 0.05, 0.02)))
        .add_resource(WindowDescriptor {
            title: "focus point".to_string(),
            width: screen_dims.x,
            height: screen_dims.y,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_stage_before(stage::UPDATE, BEFORE_UPDATE, SystemStage::serial())
        .add_startup_system(setup.system())
        //.add_system(cursor_system.system())
        .add_system_to_stage(BEFORE_UPDATE, cursor_system.system())
        .add_system(mouse_click_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
