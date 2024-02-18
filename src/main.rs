//! Demonstrates using a custom extension to the `StandardMaterial` to modify the results of the builtin pbr shader.

use bevy::scene::SceneInstance;
use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, ToonShader>,
        >::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (customize_scene_materials, customize_standard_materials),
        )
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
        ToonShaderConfig::default(),
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

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct ToonShader {
    #[texture(100)]
    #[sampler(101)]
    mask: Option<Handle<Image>>,
    #[uniform(102)]
    config: ToonShaderConfig,
}

impl MaterialExtension for ToonShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/toon_shader.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/toon_shader.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Component, ShaderType)]
struct ToonShaderConfig {
    highlight_color: Color,
    shadow_color: Color,
    rim_color: Color,
}

impl Default for ToonShaderConfig {
    fn default() -> Self {
        Self {
            highlight_color: Color::hex("FEFEFE").unwrap(),
            shadow_color: Color::hex("A1958F").unwrap(),
            rim_color: Color::WHITE,
        }
    }
}

/// Source: https://github.com/bevyengine/bevy/discussions/8533#discussioncomment-5787519
fn customize_scene_materials(
    unloaded_instances: Query<
        (Entity, Option<&SceneInstance>, &ToonShaderConfig),
        With<Handle<Scene>>,
    >,
    handles: Query<(Entity, &Handle<StandardMaterial>)>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    scene_manager: Res<SceneSpawner>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ToonShader>>>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, instance, config) in unloaded_instances.iter() {
        if let Some(instance) = instance {
            if scene_manager.instance_is_ready(**instance) {
                cmds.entity(entity).remove::<ToonShaderConfig>();
            }
            // Iterate over all entities in scene (once it's loaded)
            let handles = handles.iter_many(scene_manager.iter_instance_entities(**instance));
            for (entity, material_handle) in handles {
                let Some(material) = pbr_materials.get(material_handle) else {
                    continue;
                };
                let toon_material = materials.add(ExtendedMaterial {
                    base: material.clone(),
                    extension: ToonShader {
                        mask: Some(asset_server.load("textures/ZAtoon.png")),
                        config: config.clone(),
                    },
                });
                cmds.entity(entity)
                    .insert(toon_material)
                    .remove::<Handle<StandardMaterial>>();
            }
        }
    }
}

fn customize_standard_materials(
    with_material: Query<
        (Entity, &Handle<StandardMaterial>, &ToonShaderConfig),
        Without<Handle<Scene>>,
    >,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ToonShader>>>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, material_handle, config) in with_material.iter() {
        let Some(material) = pbr_materials.get(material_handle) else {
            continue;
        };
        let toon_material = materials.add(ExtendedMaterial {
            base: material.clone(),
            extension: ToonShader {
                mask: Some(asset_server.load("textures/ZAtoon.png")),
                config: config.clone(),
            },
        });
        cmds.entity(entity)
            .insert(toon_material)
            .remove::<Handle<StandardMaterial>>();
    }
}

fn change_color(
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, ToonShader>>>,
    time: Res<Time>,
    mut clear_color: ResMut<ClearColor>,
) {
    for (_, material) in materials.iter_mut() {
        let config = &mut material.extension.config;
        let highlights = [Color::hex("FEFEFE").unwrap(), Color::hex("8298AF").unwrap()];
        let shadows = [Color::hex("A1958F").unwrap(), Color::hex("5C6C96").unwrap()];
        let backgrounds = [Color::CYAN, Color::MIDNIGHT_BLUE];
        let frequency = 0.1;

        let lerp = (time.elapsed_seconds() * std::f32::consts::PI * frequency).sin();
        let lerp = lerp * lerp;
        let highlight_color = mix_colors(highlights[0], highlights[1], lerp);
        let shadow_color = mix_colors(shadows[0], shadows[1], lerp);
        let background_color = mix_colors(backgrounds[0], backgrounds[1], lerp);

        config.highlight_color = highlight_color;
        config.shadow_color = shadow_color;
        config.rim_color = Color::WHITE;
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
