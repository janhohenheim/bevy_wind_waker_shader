#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod prelude {
    pub use crate::{
        TimeOfDay, Weather, WindWakerShader, WindWakerShaderBuilder, WindWakerShaderPlugin,
    };
}

pub use components::{
    ExtendedMaterial, TimeOfDay, Weather, WindWakerShader, WindWakerShaderBuilder,
};
pub use plugin::WindWakerShaderPlugin;

mod components;
mod plugin;
mod systems;
