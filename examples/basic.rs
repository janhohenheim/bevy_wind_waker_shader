use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WindWakerShaderPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_things, change_color))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // sphere
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(0.0, -1.0, 0.0),
                scale: Vec3::splat(3.0),
                ..default()
            },
            ..default()
        },
        WindWakerShaderBuilder::default().build(),
    ));

    // light
    commands.spawn((PointLightBundle::default(), Rotate));

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Component)]
struct Rotate;

fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in q.iter_mut() {
        t.translation = Vec3::new(
            time.elapsed_seconds().sin(),
            0.5,
            time.elapsed_seconds().cos(),
        ) * 4.0;
    }
}

fn change_color(
    mut materials: ResMut<Assets<bevy_wind_waker_shader::ExtendedMaterial>>,
    time: Res<Time>,
    mut clear_color: ResMut<ClearColor>,
) {
    for (_, material) in materials.iter_mut() {
        let toon_shader = &mut material.extension;
        let highlights = [Color::hex("FEFEFE").unwrap(), Color::hex("8298AF").unwrap()];
        let shadows = [Color::hex("A1958F").unwrap(), Color::hex("5C6C96").unwrap()];
        let backgrounds = [Color::CYAN, Color::MIDNIGHT_BLUE];
        let frequency = 0.1;

        let lerp = (time.elapsed_seconds() * std::f32::consts::PI * frequency).sin();
        let lerp = lerp * lerp;
        let highlight_color = mix_colors(highlights[0], highlights[1], lerp);
        let shadow_color = mix_colors(shadows[0], shadows[1], lerp);
        let background_color = mix_colors(backgrounds[0], backgrounds[1], lerp);

        toon_shader.highlight_color = highlight_color;
        toon_shader.shadow_color = shadow_color;
        toon_shader.rim_color = Color::WHITE;
        clear_color.0 = background_color;
    }
}

fn mix_colors(a: Color, b: Color, t: f32) -> Color {
    Color::rgb(
        a.r() * (1.0 - t) + b.r() * t,
        a.g() * (1.0 - t) + b.g() * t,
        a.b() * (1.0 - t) + b.b() * t,
    )
}
