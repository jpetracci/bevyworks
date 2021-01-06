use bevy::{prelude::*, render::pass::ClearColor};
use rand::Rng;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(FireworkTimer(Timer::from_seconds(1.0, true)))
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup.system())
        .add_system(firework_propellant.system())
        .add_system(explode.system())
        .add_system(launcher.system())
        .run();
}

struct Firework {
    start: Vec3,
    target: Vec3,
    time_in_flight: f32,
}
struct Materials {
    mats: Vec<Handle<ColorMaterial>>,
}

struct FireworkTimer(Timer);

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // vec of the different mats that can be used. Just simple colors for now
    let mat_vec = Materials {
        mats: vec![
            materials.add(Color::GREEN.into()),
            materials.add(Color::RED.into()),
            materials.add(Color::BLUE.into()),
            materials.add(Color::YELLOW.into()),
        ],
    };

    // setup the generic components
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(mat_vec);
}

fn firework_propellant(time: Res<Time>, mut query: Query<(&mut Firework, &mut Transform)>) {
    let delta = f32::min(0.2, time.delta_seconds());

    // move the firework rocket
    for (mut firework, mut transform) in query.iter_mut() {
        firework.time_in_flight += delta;
        //println!("time in flight: {}", firework.time_in_flight);

        //transform.translation += firework.velocity * delta;
        let step = firework.time_in_flight / 3.0;
        let change = firework.start.lerp(firework.target, step);
        transform.translation = change;
    }
}

fn explode(
    commands: &mut Commands,
    materials: Res<Materials>,
    query: Query<(Entity, &Firework, &Transform)>,
) {
    for (entity, firework, transform) in query.iter() {
        // check if firework rocket reached desired hieght
        // TODO: should be random after certain height, or where mouse clicked
        if firework.time_in_flight > 3.0 {
            println!("DESPAWN");
            // save the current position
            // remove firework rocket
            commands.despawn(entity);

            // setup firework explosion
            // TODO: just a different color for now to debug
            let mat = materials.mats[0].clone_weak();

            // spawn an exploded firework
            // TODO: for now just create a large sprite at current position
            // TODO: Should fade out after short timer
            commands.spawn(SpriteBundle {
                material: mat,
                transform: Transform::from_translation(transform.translation),
                sprite: Sprite::new(Vec2::new(100.0, 100.0)),
                ..Default::default()
            });
        }
    }
}

fn launcher(
    commands: &mut Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: ResMut<FireworkTimer>,
) {
    if timer.0.tick(time.delta_seconds()).finished() {
        let mut rng = rand::thread_rng();
        let t_x: f32 = rng.gen_range(-400.0..400.0);
        let t_y: f32 = rng.gen_range(200.0..300.0);
        let s_x: f32 = rng.gen_range(-50.0..50.0);

        // create firework projectile
        // TODO: add random number of projectiles going to different spots in sky
        commands
            .spawn(SpriteBundle {
                material: materials.mats[1].clone_weak(),
                transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
                sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                ..Default::default()
            })
            .with(Firework {
                start: Vec3::new(s_x, -200.0, 0.0),
                target: Vec3::new(t_x, t_y, 0.0),
                time_in_flight: 0.0,
            });

        // reset launch timer to new value
        let rnd_time = rng.gen_range(2..6);
        timer.0.set_duration(rnd_time as f32);
    }
}
