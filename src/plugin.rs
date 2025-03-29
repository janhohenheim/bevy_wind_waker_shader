use crate::{
    components::{WindWakerShader, SHADER_HANDLE, TEXTURE_HANDLE},
    systems::{customize_scene_materials, customize_standard_materials},
};
use bevy::app::{App, Plugin, Update};
use bevy::asset::{load_internal_asset, Assets};
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::pbr::MaterialPlugin;
use bevy::prelude::{Image, Shader};
use bevy::render::render_asset::RenderAssetUsages;

/// Plugin for the Wind Waker shader.
#[non_exhaustive]
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
        app.world_mut()
            .resource_mut::<Assets<Image>>()
            .insert(TEXTURE_HANDLE.id(), img);

        app.add_plugins(MaterialPlugin::<crate::ExtendedMaterial>::default())
            .add_systems(
                Update,
                (customize_scene_materials, customize_standard_materials),
            )
            .register_type::<WindWakerShader>();
    }
}
