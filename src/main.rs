use bevy::{prelude::*, render::pass::ClearColor};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgba(0.1, 0.1, 0.1, 0.5)))
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(Color::RED.into()),
            transform: Transform::default(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        });
}
