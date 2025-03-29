# Wind Waker Shader

[![crates.io](https://img.shields.io/crates/v/bevy_wind_waker_shader)](https://crates.io/crates/bevy_wind_waker_shader)
[![docs.rs](https://docs.rs/bevy_wind_waker_shader/badge.svg)](https://docs.rs/bevy_wind_waker_shader)

A toon shader that looks like the one used for characters in The Legend of Zelda: The Wind Waker.
The main code is taken from the ideas presented in [this video](https://www.youtube.com/watch?v=mnxs6CR6Zrk).

## Showcase

Sphere:

![Sphere](https://github.com/janhohenheim/bevy_wind_waker_shader/assets/9047632/c5493f3e-fd09-4795-b62c-aa6b33a23089)

Scene throughout day:

<https://github.com/janhohenheim/bevy_wind_waker_shader/assets/9047632/80aa9851-f425-4439-88f1-558918caa9f1>

Scene in daylight:

![Scene in daylight](https://github.com/janhohenheim/bevy_wind_waker_shader/assets/9047632/664c3830-c053-408d-9444-29a10004c60e)

Scene at night:

![Scene at night](https://github.com/janhohenheim/bevy_wind_waker_shader/assets/9047632/4e483e73-2c8e-4a0c-a9cf-c4ea8b182a4f)

## Functionality

The shader has the following properties:

- It is a toon shader with only two colors: the highlight and the shadow.
- The edge between the two colors is not entirely hard but has an ever-so-slight gradient.
- The color palette used is based on the time of day and the weather.
- The model has a rim highlight on the edge to make it pop.

All colors and the texture mask are taken from The Legend of Zelda: The Wind Waker.

Differences to The Wind Waker:

- This shader supports multiple light sources, like in Breath of the Wild. The original Wind Waker only supports a
  single light source.
- The rim highlight also comes from Breath of the Wild.
- The Wind Waker uses even more weather conditions, but I find most of them too specific to include in this shader.

Keep in mind this shader only replicates what is seen on the *characters* in The Wind Waker, not the environment!

## Example

```rust,no_run
use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WindWakerShaderPlugin::default()))
        .add_systems(Startup, spawn_character)
        .run();
}

fn spawn_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/Fox.glb"),
            ..default()
        },
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Afternoon)
            .weather(Weather::Sunny)
            .build(),
    ));
}
```

## Compatibility

| bevy        | bevy_wind_waker_shader |
|-------------|------------------------|
| 0.15        | 0.3                    |
| 0.14        | 0.2                    |
| 0.13        | 0.1                    |
