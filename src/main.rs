use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    render::pass::ClearColor,
    window::CursorMoved,
};
use rand::Rng;

struct Firework {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    time: f32,
    shell: bool,
}

impl Firework {
    fn add_force(&mut self, force: Vec3) {
        self.acc += force;
    }

    fn update(&mut self, delta: f32) {
        self.vel += self.acc * delta;
        self.pos += self.vel;
        self.time -= delta;

        //zero out the acceleration as it has been applied to the velocity
        self.acc = Vec3::zero();
    }
}

struct Materials {
    mats: Vec<Handle<ColorMaterial>>,
}

struct FireworkTimer(Timer);

struct MousePos(Vec2);

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(FireworkTimer(Timer::from_seconds(1.0, true)))
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_resource(MousePos(Vec2::new(0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(firework_update.system())
        .add_system(explode.system())
        .add_system(launcher.system())
        .add_system(mouse_movement_detector.system())
        .run();
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
}

// need to save the mouse position manually
fn mouse_movement_detector(
    mut mouse_pos: ResMut<MousePos>,
    mut state: Local<State>,
    windows: Res<Windows>,
    cursor_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_events) {
        let window = windows.get(event.id).unwrap();
        // game coords are 0 0 in middle of window
        // mouse coords are 0 0 in bottom left of winodw
        mouse_pos.0.x = event.position.x - (window.width() as f32) / 2.0;
        mouse_pos.0.y = event.position.y - (window.height() as f32) / 2.0;
    }
}

fn firework_update(time: Res<Time>, mut query: Query<(&mut Firework, &mut Transform)>) {
    let delta = f32::min(0.2, time.delta_seconds());

    // update the firework
    for (mut firework, mut transform) in query.iter_mut() {
        // add some "gravity"
        firework.add_force(Vec3::new(0.0, -0.05, 0.0));
        firework.update(delta);
        transform.translation = firework.pos;
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
        if firework.time <= 0.0 {
            // remove firework
            commands.despawn(entity);

            // spawn an exploded firework if a shell
            if firework.shell {
                // setup firework explosion
                // TODO: just a different color for now to debug
                let mut rng = rand::thread_rng();
                let mat_index = rng.gen_range(0..(materials.mats.len()));
                let mat = materials.mats[mat_index].clone_weak();
                boom(commands, mat, transform);
            }
        }
    }
}

fn boom(commands: &mut Commands, material: Handle<ColorMaterial>, transform: &Transform) {
    //TODO: Should fade out over time
    let size = 4.0;
    let num = 50;
    let mut rng = rand::thread_rng();

    for i in 0..num + 1 {
        let mat = material.clone();
        let vx = rng.gen_range(-0.10..0.10);
        let vy = rng.gen_range(-0.10..0.10);

        commands
            .spawn(SpriteBundle {
                material: mat,
                transform: Transform::from_translation(transform.translation),
                sprite: Sprite::new(Vec2::new(size, size)),
                ..Default::default()
            })
            .with(Firework {
                pos: transform.translation,
                vel: Vec3::new(vx, vy, 0.0),
                acc: Vec3::new(0.0, 0.0, 0.0),
                time: 1.5,
                shell: false,
            });
    }
}

fn launcher(
    commands: &mut Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut state: Local<State>,
    mouse_pos: Res<MousePos>,
    mouse_button_events: Res<Events<MouseButtonInput>>,
    mut timer: ResMut<FireworkTimer>,
) {
    // spawn from mouse click
    // TODO: Need to calc vel and time from mouse click (distance and direction)
    // for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
    //     if event.button == MouseButton::Left && event.state == ElementState::Released {
    //         println!("{:?}", mouse_pos.0);

    //         // TODO: move to function
    //         let mut rng = rand::thread_rng();
    //         let t_x: f32 = mouse_pos.0.x;
    //         let t_y: f32 = mouse_pos.0.y;
    //         let s_x: f32 = rng.gen_range(-50.0..50.0);

    //         commands
    //             .spawn(SpriteBundle {
    //                 material: materials.mats[1].clone_weak(),
    //                 transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
    //                 sprite: Sprite::new(Vec2::new(5.0, 5.0)),
    //                 ..Default::default()
    //             })
    //             .with(Firework {
    //                 pos: Vec3::new(s_x, -200.0, 0.0),
    //                 vel: Vec3::new(1.0, 1.0, 0.0),
    //                 acc: Vec3::zero(),
    //                 time: 3.0,
    //             });
    //     }
    // }

    // spawn randomly
    if timer.0.tick(time.delta_seconds()).finished() {
        let mut rng = rand::thread_rng();
        let vx: f32 = rng.gen_range(-0.15..0.15);
        let vy: f32 = rng.gen_range(0.15..0.2);
        let pos_x: f32 = rng.gen_range(-50.0..50.0);

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
                pos: Vec3::new(pos_x, -200.0, 0.0),
                vel: Vec3::new(vx, vy, 0.0),
                acc: Vec3::zero(),
                time: 2.0,
                shell: true,
            });

        // reset launch timer to new value
        let rnd_time: f32 = rng.gen_range(0.0..0.5);
        timer.0.set_duration(rnd_time as f32);
    }
}
