#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![doc = include_str!("../readme.md")]

pub mod prelude {
    //! Everything you need to get started with the Wind Waker shader. For the main feature, see [WindWakerShaderBuilder].
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
