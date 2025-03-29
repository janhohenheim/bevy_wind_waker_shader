use bevy::asset::{weak_handle, Asset, Handle};
use bevy::pbr::{MaterialExtension, StandardMaterial};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub(crate) const SHADER_HANDLE: Handle<Shader> =
    weak_handle!("ddeed264-efde-495e-9159-4ac3db07f9f8");
pub(crate) const TEXTURE_HANDLE: Handle<Image> =
    weak_handle!("1af26f3e-5605-4723-a036-dc83f357c7d8");

/// The type of the material that will be inserted for you after you insert the [`WindWakerShader`] via the [`WindWakerShaderBuilder`] into an entity.
pub type ExtendedMaterial = bevy::pbr::ExtendedMaterial<StandardMaterial, WindWakerShader>;

/// Build via the [`WindWakerShaderBuilder`] and insert into an entity to give it a shader that looks
/// like the one used by characters in The Legend of Zelda: The Wind Waker.
///
/// After insertion, the shader will be moved into the [`ExtendedMaterial`] of the entity.
#[derive(Asset, AsBindGroup, PartialEq, Debug, Clone, Component, Reflect)]
#[reflect(PartialEq)]
pub struct WindWakerShader {
    #[texture(100)]
    #[sampler(101)]
    mask: Handle<Image>,
    /// The parts of the model that are facing the light source and are not in shadow.
    #[uniform(102)]
    pub highlight_color: LinearRgba,
    /// The parts of the model that are not facing the light source and are in shadow.
    #[uniform(103)]
    pub shadow_color: LinearRgba,
    /// The color of the edge of the model, which gets a slight specular highlight to make the model pop.
    #[uniform(104)]
    pub rim_color: LinearRgba,
}

impl Default for WindWakerShader {
    fn default() -> Self {
        WindWakerShaderBuilder::default().build()
    }
}

impl MaterialExtension for WindWakerShader {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_HANDLE.into()
    }
}

/// Builds a new [`WindWakerShader`] by setting the parameters to look like those in The Legend of Zelda: The Wind Waker.
/// After insertion, the shader will be moved into the [`ExtendedMaterial`] of the entity.
/// If the entity in question is a [`Scene`](bevy::prelude::Scene), this is done for all the entities inside the scene.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use bevy_wind_waker_shader::prelude::*;
///
/// fn spawn_with_wind_waker_shader(mut commands: Commands, asset_server: Res<AssetServer>) {
///     commands.spawn((
///         SceneRoot(asset_server.load("models/Fox.glb")),
///         WindWakerShaderBuilder::default()
///             .time_of_day(TimeOfDay::Afternoon)
///             .weather(Weather::Sunny)
///             .build(),
///    ));
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct WindWakerShaderBuilder {
    time_of_day: TimeOfDay,
    weather: Weather,
    override_highlight_color: Option<Color>,
    override_shadow_color: Option<Color>,
    override_rim_color: Option<Color>,
}

impl WindWakerShaderBuilder {
    /// Uses the color palette associated with the given time of day in The Legend of Zelda: The Wind Waker.
    /// Note that the [weather](WindWakerShaderBuilder::weather) will modify the colors.
    ///
    /// The default time of day is [TimeOfDay::Day].
    pub fn time_of_day(mut self, time: TimeOfDay) -> Self {
        self.time_of_day = time;
        self
    }

    /// Modifies the color palette associated with the [time of day](WindWakerShaderBuilder::time_of_day) by the given weather.
    ///
    /// The default weather is [Weather::Sunny].
    pub fn weather(mut self, weather: Weather) -> Self {
        self.weather = weather;
        self
    }

    /// Overrides the highlight color with the given color. Highlights are the parts of the model that are facing the light source and are not in shadow.
    /// This overrides both the [time of day](WindWakerShaderBuilder::time_of_day) and [weather](WindWakerShaderBuilder::weather) settings.
    pub fn override_highlight_color(mut self, color: Color) -> Self {
        self.override_highlight_color = Some(color);
        self
    }

    /// Overrides the shadow color with the given color. Shadows are the parts of the model that are not facing the light source.
    /// This overrides both the [time of day](WindWakerShaderBuilder::time_of_day) and [weather](WindWakerShaderBuilder::weather) settings.
    pub fn override_shadow_color(mut self, color: Color) -> Self {
        self.override_shadow_color = Some(color);
        self
    }

    /// Overrides the rim color with the given color. The rim is the edge of the model, which gets a slight specular highlight to make the model pop.
    /// This overrides both the [time of day](WindWakerShaderBuilder::time_of_day) and [weather](WindWakerShaderBuilder::weather) settings.
    pub fn override_rim_color(mut self, color: Color) -> Self {
        self.override_rim_color = Some(color);
        self
    }

    /// Builds the [`WindWakerShader`] with the given settings. Note that after insertion, the shader will be moved into the [`ExtendedMaterial`] of the entity.
    pub fn build(self) -> WindWakerShader {
        let (highlight_hex, shadow_hex) = match (self.time_of_day, self.weather) {
            (TimeOfDay::Dusk, Weather::Sunny) => ("A19AA3", "746676"),
            (TimeOfDay::Dusk, Weather::Rainy) => ("90887A", "746676"),
            (TimeOfDay::Morning, Weather::Sunny) => ("F0EAE3", "BCB7CB"),
            (TimeOfDay::Morning, Weather::Rainy) => ("B8BDB8", "9AA494"),
            (TimeOfDay::Day, Weather::Sunny) => ("FFFFFF", "A39892"),
            (TimeOfDay::Day, Weather::Rainy) => ("ADBBB7", "8E978D"),
            (TimeOfDay::Afternoon, Weather::Sunny) => ("D8C37F", "B09070"),
            (TimeOfDay::Afternoon, Weather::Rainy) => ("999187", "888177"),
            (TimeOfDay::Evening, Weather::Sunny) => ("8D8C9A", "7E7885"),
            (TimeOfDay::Evening, Weather::Rainy) => ("8E877D", "7A7368"),
            (TimeOfDay::Night, Weather::Sunny) => ("879EB5", "5D6E99"),
            (TimeOfDay::Night, Weather::Rainy) => ("4B6690", "4C595A"),
        };
        let highlight_color = self
            .override_highlight_color
            .unwrap_or_else(|| Srgba::hex(highlight_hex).unwrap().into());
        let shadow_color = self
            .override_shadow_color
            .unwrap_or_else(|| Srgba::hex(shadow_hex).unwrap().into());
        let rim_color = self.override_rim_color.unwrap_or(Color::WHITE);
        WindWakerShader {
            mask: TEXTURE_HANDLE.clone(),
            highlight_color: highlight_color.into(),
            shadow_color: shadow_color.into(),
            rim_color: rim_color.into(),
        }
    }
}

/// The time of day used for the color palette in the [`WindWakerShaderBuilder`].
/// Note that this does not have to correspond to any actual time settings in your game.
/// Rather, think of this as "mood categories" that you can use to set the color palette.
#[derive(Debug, Clone, Copy, Default)]
#[allow(missing_docs)]
pub enum TimeOfDay {
    Dusk,
    Morning,
    #[default]
    Day,
    Afternoon,
    Evening,
    Night,
}

impl TimeOfDay {
    /// Returns the next time of day in the cycle.
    pub fn next(self) -> Self {
        match self {
            TimeOfDay::Dusk => TimeOfDay::Morning,
            TimeOfDay::Morning => TimeOfDay::Day,
            TimeOfDay::Day => TimeOfDay::Afternoon,
            TimeOfDay::Afternoon => TimeOfDay::Evening,
            TimeOfDay::Evening => TimeOfDay::Night,
            TimeOfDay::Night => TimeOfDay::Dusk,
        }
    }
}

/// The weather used for the color palette in the [`WindWakerShaderBuilder`].
/// Note that this does not have to correspond to any actual weather settings in your game.
/// Rather, think of this as "mood categories" that you can use to set the color palette.
#[derive(Debug, Clone, Copy, Default)]
#[allow(missing_docs)]
pub enum Weather {
    #[default]
    Sunny,
    Rainy,
}
