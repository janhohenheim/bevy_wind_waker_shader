use bevy::asset::load_internal_asset;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::texture::{CompressedImageFormats, ImageSampler, ImageType};
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
        load_internal_asset!(
            app,
            SHADER_HANDLE,
            "assets/toon_shader.wgsl",
            Shader::from_wgsl
        );

        let buffer = include_bytes!("assets/ZAtoon.png");
        let extension = ImageType::Extension("png");
        let compression = CompressedImageFormats::default();
        let is_srgb = false;
        let sampler = ImageSampler::default();
        let render_asset_usages = RenderAssetUsages::RENDER_WORLD;
        let img = Image::from_buffer(
            buffer,
            extension,
            compression,
            is_srgb,
            sampler,
            render_asset_usages,
        )
        .unwrap();
        app.world
            .resource_mut::<Assets<Image>>()
            .insert(TEXTURE_HANDLE, img);

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
    mask: Handle<Image>,
    #[uniform(102)]
    pub config: WindWakerShader,
}

impl MaterialExtension for WindWakerShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }
}

impl From<WindWakerShader> for WindWakerShaderMaterial {
    fn from(config: WindWakerShader) -> Self {
        Self {
            mask: TEXTURE_HANDLE.clone(),
            config,
        }
    }
}

const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(4_326_875_112_478_868_553);
const TEXTURE_HANDLE: Handle<Image> = Handle::weak_from_u128(8_324_623_845_322_188_856);

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
                    extension: WindWakerShaderMaterial::from(config.clone()),
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
) {
    for (entity, material_handle, config) in with_material.iter() {
        let Some(material) = pbr_materials.get(material_handle) else {
            continue;
        };
        let toon_material = materials.add(ExtendedMaterial {
            base: material.clone(),
            extension: WindWakerShaderMaterial::from(config.clone()),
        });
        cmds.entity(entity)
            .insert(toon_material)
            .remove::<Handle<StandardMaterial>>();
    }
}
