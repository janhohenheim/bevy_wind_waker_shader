use bevy::core_pipeline::fxaa::Fxaa;
use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WindWakerShaderPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, change_color)
        .insert_resource(Msaa::default())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // objects
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("FlightHelmet/FlightHelmet.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(-1.5, -1.0, 0.0),
                scale: Vec3::splat(4.0),
                ..default()
            },
            ..default()
        },
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
    ));
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Fox.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(1.5, -1.0, 0.0),
                scale: Vec3::splat(0.03),
                ..default()
            }
            .looking_at(Vec3::new(2.0, -2.5, -5.0), Vec3::Y),
            ..default()
        },
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(2.0, 0.5, 2.0),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Fxaa::default(),
    ));
}

fn change_color(
    models: Query<&Handle<bevy_wind_waker_shader::ExtendedMaterial>>,
    mut materials: ResMut<Assets<bevy_wind_waker_shader::ExtendedMaterial>>,
    time: Res<Time>,
    mut time_since_change: Local<f32>,
    mut time_of_day: Local<TimeOfDay>,
) {
    // Change the color every second
    *time_since_change += time.delta_seconds();
    if *time_since_change < 1.0 {
        return;
    }
    *time_since_change = 0.0;

    *time_of_day = time_of_day.next();
    for handle in models.iter() {
        let material = materials.get_mut(handle).unwrap();
        material.extension = WindWakerShaderBuilder::default()
            .time_of_day(*time_of_day)
            .build();
    }
}
