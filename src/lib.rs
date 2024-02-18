use bevy::scene::SceneInstance;
use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::*,
};

pub mod prelude {
    pub use crate::{WindWakerShader, WindWakerShaderPlugin};
}

#[derive(Debug, Default, Clone, Copy)]
pub struct WindWakerShaderPlugin;

impl Plugin for WindWakerShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, WindWakerShaderMaterial>,
        >::default())
            .add_systems(
                Update,
                (customize_scene_materials, customize_standard_materials),
            )
            .register_type::<WindWakerShader>();
    }
}

pub type Material = ExtendedMaterial<StandardMaterial, WindWakerShaderMaterial>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct WindWakerShaderMaterial {
    #[texture(100)]
    #[sampler(101)]
    mask: Option<Handle<Image>>,
    #[uniform(102)]
    pub config: WindWakerShader,
}

impl MaterialExtension for WindWakerShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/toon_shader.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/toon_shader.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Component, ShaderType)]
pub struct WindWakerShader {
    pub highlight_color: Color,
    pub shadow_color: Color,
    pub rim_color: Color,
}

impl Default for WindWakerShader {
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
        (Entity, Option<&SceneInstance>, &WindWakerShader),
        With<Handle<Scene>>,
    >,
    handles: Query<(Entity, &Handle<StandardMaterial>)>,
    pbr_materials: Res<Assets<StandardMaterial>>,
    scene_manager: Res<SceneSpawner>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WindWakerShaderMaterial>>>,
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, instance, config) in unloaded_instances.iter() {
        if let Some(instance) = instance {
            if scene_manager.instance_is_ready(**instance) {
                cmds.entity(entity).remove::<WindWakerShader>();
            }
            // Iterate over all entities in scene (once it's loaded)
            let handles = handles.iter_many(scene_manager.iter_instance_entities(**instance));
            for (entity, material_handle) in handles {
                let Some(material) = pbr_materials.get(material_handle) else {
                    continue;
                };
                let toon_material = materials.add(ExtendedMaterial {
                    base: material.clone(),
                    extension: WindWakerShaderMaterial {
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
        (Entity, &Handle<StandardMaterial>, &WindWakerShader),
        Without<Handle<Scene>>,
    >,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WindWakerShaderMaterial>>>,
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
            extension: WindWakerShaderMaterial {
                mask: Some(asset_server.load("textures/ZAtoon.png")),
                config: config.clone(),
            },
        });
        cmds.entity(entity)
            .insert(toon_material)
            .remove::<Handle<StandardMaterial>>();
    }
}
