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
            ExtendedMaterial<StandardMaterial, MyExtension>,
        >::default())
        .insert_resource(ClearColor(Color::SEA_GREEN))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_things, customize_scene_materials))
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
        CustomizeMaterial,
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
struct MyExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    quantize_steps: u32,
    #[texture(101)]
    #[sampler(102)]
    mask: Option<Handle<Image>>,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
}

#[derive(Component)]
struct CustomizeMaterial;

/// Source: https://github.com/bevyengine/bevy/discussions/8533#discussioncomment-5787519
fn customize_scene_materials(
    unloaded_instances: Query<(Entity, &SceneInstance), With<CustomizeMaterial>>,
    handles: Query<(Entity, &Handle<StandardMaterial>)>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    scene_manager: Res<SceneSpawner>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, instance) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).remove::<CustomizeMaterial>();
        }
        // Iterate over all entities in scene (once it's loaded)
        let handles = handles.iter_many(scene_manager.iter_instance_entities(**instance));
        for (entity, material_handle) in handles {
            let Some(material) = pbr_materials.get(material_handle) else {
                continue;
            };
            let custom = materials.add(ExtendedMaterial {
                base: material.clone(),
                extension: MyExtension {
                    quantize_steps: 3,
                    mask: Some(asset_server.load("textures/ZAtoon.png")),
                },
            });
            cmds.entity(entity)
                .insert(custom)
                .remove::<Handle<StandardMaterial>>();
        }
    }
}
