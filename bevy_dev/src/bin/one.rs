use bevy::{
    input::{keyboard::KeyCode, Input},
    input::mouse::{MouseButtonInput},
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    //window::WindowMode,
    prelude::*
};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "one".to_string(),
            width: 800.,
            height: 600.,
            // mode: WindowMode::BorderlessFullscreen,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        //.add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        .add_system(keyboard_input_system.system())
        .add_system(print_mouse_events_system.system())
        .add_system(text_update_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct Rotator;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds());
    }
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        println!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        println!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        println!("'A' just released");
    }
}


#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut state: Local<State>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        println!("{:?}", event);
    }

    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        println!("{:?}", event);
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.0}", average);
            }
        }
    }
}

/// set up a simple scene with a "parent" cube and a "child" cube
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials3d: ResMut<Assets<StandardMaterial>>,
    mut materials2d: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    // Tell the asset server to watch for asset changes on disk:
    //asset_server.watch_for_changes().unwrap();

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 2.0 }));
    let cube_material_handle = materials3d.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    let texture_handle = asset_server.load("textures/bevy.png");

    commands
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(5.0, 10.0, 10.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })

        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, -4.0)),
            ..Default::default()
        })
        
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials3d.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })

        // parent cube
        .spawn(PbrBundle {
            mesh: cube_handle.clone(),
            material: cube_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .with(Rotator)
        .with_children(|parent| {
            // child cube
            parent.spawn(PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
                ..Default::default()
            });
        })

        // 2d camera
        .spawn(Camera2dBundle::default())

        .spawn(SpriteBundle {
            material: materials2d.add(texture_handle.into()),
            sprite: Sprite::new(Vec2::new(128.0, 128.0)),
            transform: Transform::from_translation(Vec3::new(-200.0, 250.0, 0.0)),
            ..Default::default()
        })

        // 2d camera - UI
        .spawn(CameraUiBundle::default())

        // fps text
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}
