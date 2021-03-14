use bevy::prelude::*;

#[derive(Debug)]
struct AnimateAlpha {
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
    //commands: &mut Commands,
    time: Res<Time>,
    mut q_anim: Query<(Entity, &AnimateAlpha, &mut Sprite), With<AnimateAlpha>>,
    q_mat: Query<&Handle<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //mut q_trans: Query<&mut Transform>,
    //mut q_sprite: Query<&mut Sprite>,
) {
    let t = time.seconds_since_startup();
    for (ent, aa, mut spr) in q_anim.iter_mut() {
        let t0 = aa.start_t;
        let t1 = t0 + aa.duration;
        if t > t1 {
            // TODO delete AnimateAlpha and set keys to their target value
            continue;
            //aa.remove();
            //commands.remove_one(aa);
        }
        let r: f32 = ((t - t0) / aa.duration) as f32;
        //println!("{:.2}", r);
        if let Ok(material_handle) = q_mat.get(ent) {
            let mut material = materials.get_mut(&*material_handle).unwrap();
            material.color = Color::rgba(1., 1., 1., r);
        }
        spr.size = Vec2::one() * 200. * r;
        //println!("{:?}", spr);
        /* if let Ok(sprite_handle) = q_sprite.get_mut(ent) {

            //let mut sprite = sprite_handle.get_mut();
            //let mut sprite = *sprite;
            println!("{:?}", sprite);
            //trans.scale = Vec3::one() * r;
        }
        if let Ok(trans) = q_trans.get_mut(ent) {
            //let mut trans = trans_handle.get_mut();
            let mut trans = *trans;
            trans.scale = Vec3::one() * r;
        } */
        //let trans = trans
        //let trans = (&*trans).unwrap();
        //let mut trans = *trans;
        //println!("t0: {:?}", trans);
        //trans.scale = Vec3::one() * r;
        //if let Ok(trans_handle) = q_trans.get_mut(ent) {
        //let mut trans = *trans_handle;
        //println!("t0: {:?}", trans);
        //trans.scale *= r;
        //trans.scale = Vec3::one() * r;
        //trans.scale.x = r;
        //trans.scale.y = r;
        //trans.scale.z = r;
        //trans.translation = Vec3::zero();
        //println!("t1: {:?}", trans);
        //}
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
        .with(AnimateAlpha {
            start_t: 1.,
            duration: 2.,
        });
}
