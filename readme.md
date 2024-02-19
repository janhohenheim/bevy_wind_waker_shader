
# Wind Waker Shader

[![crates.io](https://img.shields.io/crates/v/bevy_wind_waker_shader)](https://crates.io/crates/bevy_wind_waker_shader)
[![docs.rs](https://docs.rs/bevy_wind_waker_shader/badge.svg)](https://docs.rs/bevy_wind_waker_shader)

A toon shader that looks like the one used for characters in The Legend of Zelda: The Wind Waker. 
The main code is taken from the ideas presented in [this video](https://www.youtube.com/watch?v=mnxs6CR6Zrk).


## Showcase

Sphere:

![image](https://github.com/janhohenheim/wind-waker-shader/assets/9047632/befb4618-4a68-4f83-a019-d07362e77b43)


Scene throughout day:

<https://github.com/janhohenheim/bevy_wind_waker_shader/assets/9047632/9668b849-cd2d-4e0f-a302-6638e5e03863>


Scene in daylight:

![image](https://github.com/janhohenheim/wind-waker-shader/assets/9047632/ad489828-35a9-4e35-8c0b-a4ecf57d8296)


Scene at night:

![image](https://github.com/janhohenheim/wind-waker-shader/assets/9047632/140684a4-bd7b-49fe-9af8-837ccfbbd6b5)


## Functionality

The shader has the following properties:
- It is a toon shader with only two colors: the highlight and the shadow.
- The color palette used is based on the time of day and the weather.
- The model has a rim highlight on the edge to make it pop.

All colors and the texture mask are taken from The Legend of Zelda: The Wind Waker.

Differences to The Wind Waker:
- This shader supports multiple light sources, like in Breath of the Wild. The original Wind Waker only supports a single light source.
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
| bevy | bevy_wind_waker_shader |
|------|------------------------|
| 0.13 | 0.1                    |

