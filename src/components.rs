use bevy::asset::{Asset, Handle};
use bevy::pbr::{MaterialExtension, StandardMaterial};
use bevy::prelude::{Color, Component, Image, Reflect, Shader};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};

pub(crate) const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(4_326_875_112_478_868_553);
pub(crate) const TEXTURE_HANDLE: Handle<Image> = Handle::weak_from_u128(8_324_623_845_322_188_856);

pub type ExtendedMaterial = bevy::pbr::ExtendedMaterial<StandardMaterial, WindWakerShaderMaterial>;

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
