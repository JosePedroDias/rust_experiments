use super::{
    components::*, game_logic::*, resources::*, shapes::rect::build_rect_uvs,
    shapes::stroked_rect::build_stroked_rect,
};
use bevy::{prelude::*, render::mesh::Mesh};
use open;
use std::mem;

pub fn mouse_handling_system(
    commands: &mut Commands,
    ev_cursor: Res<Events<CursorMoved>>,
    mut evr_cursor: Local<EventReader<CursorMoved>>,
    mouse_button_input: Res<Input<MouseButton>>,
    wnds: Res<Windows>,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_camera: Query<&Transform, With<MainCamera>>,
    q_tile: Query<(Entity, &TileData), With<TileData>>,
) {
    let camera_transform = q_camera.iter().next().unwrap();
    let mut pos_wld: Option<Vec4> = None;

    for ev in evr_cursor.iter(&ev_cursor) {
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        //println!("size: {} x {}", size.x, size.y);
        game_state.screen_dims = size;
        let p = ev.position - size / 2.0;
        pos_wld = Some(camera_transform.compute_matrix() * p.extend(0.0).extend(1.0));
    }

    if pos_wld.is_some() {
        let pos_wld = pos_wld.unwrap();
        game_state.mouse_pos.x = pos_wld.x;
        game_state.mouse_pos.y = pos_wld.y;
        //println!("POS: {:.0}x{:.0}", pos_wld.x, pos_wld.y);
    }

    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    let pw = game_state.mouse_pos;
    let mut hovered_ent = None;

    for (entity, td) in q_tile.iter() {
        let center = td.center;
        let dims = td.dims;

        let x0 = center.x - dims.x * 0.5;
        let x1 = center.x + dims.x * 0.5;
        let y0 = center.y - dims.y * 0.5;
        let y1 = center.y + dims.y * 0.5;

        if pw.x > x0 && pw.x < x1 && pw.y > y0 && pw.y < y1 {
            hovered_ent = Some(entity);
            break;
        }
    }

    game_state.hovered_entity = hovered_ent;
    //println!("hovered: {:?}", hovered_ent);

    if hovered_ent.is_none() && game_state.mouse_pos.y < 0. {
        open::that(game_state.image_url.clone()).ok();
        return;
    }

    if game_state.hovered_entity.is_some() {
        if game_state.selected_entity.is_some() {
            let ent1 = game_state.hovered_entity.unwrap();
            let ent2 = game_state.selected_entity.unwrap();
            if ent1 == ent2 {
                println!("unselected both");
            } else {
                println!("swap: {:?} <-> {:?}", ent1, ent2);
                let mut td1: Option<TileData> = None;
                let mut td2: Option<TileData> = None;

                for (entity, td) in q_tile.iter() {
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
                    let mesh = meshes.add(build_rect_uvs(dims, uvs));
                    commands
                        .spawn(generate_tile_bundle(mesh, mat, td.center))
                        .with(td.clone());
                }
            }
            game_state.hovered_entity = None;
            game_state.selected_entity = None;
        } else {
            game_state.selected_entity = game_state.hovered_entity;
            game_state.hovered_entity = None;
            println!("selected 1: {:?}", game_state.selected_entity.unwrap());
        }
    }
}

pub fn game_setup_system(
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

    let mat2 = materials.add(Color::rgba(1., 0., 1., 0.5).into());
    game_state.stroked_material_handle = Some(mat2.clone());

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
            let mesh = meshes.add(build_rect_uvs(dims, uvs));
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

    let mesh2 = meshes.add(build_stroked_rect(Vec2::new(200., 150.), 2., 2.));
    commands
        .spawn(generate_tile_bundle(
            mesh2,
            mat2.clone(),
            Vec3::new(0., 0., 2.),
        ))
        .with(StrokedTile);

    // 2d camera - UI
    commands
        .spawn(CameraUiBundle::default())
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            text: Text {
                value: game_state.image_credits.clone(),
                font: asset_server.load(FONT),
                style: TextStyle {
                    font_size: 14.0,
                    color: Color::rgba(1., 1., 1., 0.25),
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(Credits);
}
