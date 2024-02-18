pub mod prelude {
    pub use crate::{WindWakerShader, WindWakerShaderPlugin};
}

pub use components::{ExtendedMaterial, WindWakerShader, WindWakerShaderMaterial};
pub use plugin::WindWakerShaderPlugin;

mod components;
mod plugin;
mod systems;
