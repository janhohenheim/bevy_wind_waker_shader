use bevy::anti_alias::fxaa::Fxaa;
use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WindWakerShaderPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_light)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // objects
    commands.spawn((
        SceneRoot(asset_server.load("FlightHelmet/FlightHelmet.gltf#Scene0")),
        Transform {
            translation: Vec3::new(-1.5, -1.0, 0.0),
            scale: Vec3::splat(4.0),
            ..default()
        },
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
    ));
    commands.spawn((
        SceneRoot(asset_server.load("Fox.glb#Scene0")),
        Transform {
            translation: Vec3::new(1.5, -1.0, 0.0),
            scale: Vec3::splat(0.03),
            ..default()
        }
        .looking_at(Vec3::new(2.0, -2.5, -5.0), Vec3::Y),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
    ));

    // light
    commands.spawn(PointLight::default());

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Fxaa::default(),
    ));
}

fn rotate_light(mut q: Query<&mut Transform, With<PointLight>>, time: Res<Time>) {
    for mut t in q.iter_mut() {
        t.translation = Vec3::new(time.elapsed_secs().sin(), 0.5, time.elapsed_secs().cos()) * 4.0;
    }
}
