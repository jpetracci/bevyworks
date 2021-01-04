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
    velocity: Vec3,
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

fn firework_propellant(time: Res<Time>, mut query: Query<(&Firework, &mut Transform)>) {
    let delta = f32::min(0.2, time.delta_seconds());

    // move the firework rocket
    for (firework, mut transform) in query.iter_mut() {
        transform.translation += firework.velocity * delta;
    }
}

fn explode(commands: &mut Commands, materials: Res<Materials>, query: Query<(Entity, &Transform)>) {
    for (firework, transform) in query.iter() {
        // check if firework rocket reached desired hieght
        // TODO: should be random after certain height, or where mouse clicked
        if transform.translation.y > 300.0 {
            // save the current position
            // remove firework rocket
            commands.despawn(firework);

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
                velocity: 100.0 * Vec3::new(0.0, 0.5, 0.0).normalize(),
            });

        // reset launch timer to new value
        let mut rng = rand::thread_rng();
        let rnd_time = rng.gen_range(2..6);
        timer.0.set_duration(rnd_time as f32);
    }
}
