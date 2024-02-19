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
    // sphere
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("FlightHelmet/FlightHelmet.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(0.0, -1.0, 0.0),
                scale: Vec3::splat(3.0),
                ..default()
            },
            ..default()
        },
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
    ));

    // light
    commands.spawn(PointLightBundle::default());

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_light(mut q: Query<&mut Transform, With<PointLight>>, time: Res<Time>) {
    for mut t in q.iter_mut() {
        t.translation = Vec3::new(
            time.elapsed_seconds().sin(),
            0.5,
            time.elapsed_seconds().cos(),
        ) * 4.0;
    }
}
