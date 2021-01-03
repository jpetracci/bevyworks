use bevy::{prelude::*, render::pass::ClearColor};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgba(0.1, 0.1, 0.1, 0.5)))
        .add_startup_system(setup.system())
        .add_system(firework_propellant.system())
        .add_system(explode.system())
        .run();
}

struct Firework {
    velocity: Vec3,
}
struct Materials {
    mats: Vec<Handle<ColorMaterial>>,
}

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

    // TODO: this needs to be in a rocket spawner.
    let mat = materials.add(Color::RED.into());
    let pos = Vec3::new(0.0, -200.0, 0.0);
    blast_off(commands, mat, pos);
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
            // TODO: for now just blast off from current position
            blast_off(commands, mat, transform.translation);
        }
    }
}

fn blast_off(commands: &mut Commands, mat: Handle<ColorMaterial>, pos: Vec3) {
    commands
        .spawn(SpriteBundle {
            material: mat,
            transform: Transform::from_translation(pos),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Firework {
            velocity: 100.0 * Vec3::new(0.0, 0.5, 0.0).normalize(),
        });
}
