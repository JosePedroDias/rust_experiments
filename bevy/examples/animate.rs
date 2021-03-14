use bevy::prelude::*;

#[derive(Debug)]
struct Animate {
    start_t: f64,
    duration: f64,
}

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.4)))
        .add_resource(WindowDescriptor {
            title: "animate".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn animate_system(
    commands: &mut Commands,
    time: Res<Time>,
    mut q_anim: Query<(Entity, &Animate, &mut Sprite, &mut Transform), With<Animate>>,
    q_mat: Query<&Handle<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let t = time.seconds_since_startup();
    for (ent, aa, mut spr, mut tr) in q_anim.iter_mut() {
        let mut t_ = t;
        let t0 = aa.start_t;
        let t1 = t0 + aa.duration;
        let mut to_kill = false;
        if t_ > t1 {
            to_kill = true;
            t_ = t1;
        }
        let r: f32 = ((t_ - t0) / aa.duration) as f32;
        //println!("{:.3}", r);
        if let Ok(material_handle) = q_mat.get(ent) {
            let mut material = materials.get_mut(&*material_handle).unwrap();
            material.color = Color::rgba(1., 1., 1., r);
        }
        spr.size = Vec2::one() * 200. * r;

        //tr.scale = Vec3::one() * r;
        tr.translation.x = -150. + r * 300.;

        if to_kill {
            commands.remove_one::<Animate>(ent);
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let uv_tex = asset_server.load("textures/uvs/1.png");

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(uv_tex.clone().into()),
            sprite: Sprite::new(Vec2::new(200., 200.)), // scales...
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        })
        .with(Animate {
            start_t: 1.,
            duration: 2.,
        });
}
