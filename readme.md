# Wind Waker Shader

A shader that looks like the one used for characters in The Legend of Zelda: The Wind Waker. 
The main code is taken from the ideas presented in [this video](https://www.youtube.com/watch?v=mnxs6CR6Zrk).

The shader has the following properties:
- Is is a toon shader with only two colors: the highlight and the shadow.
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
    App::build()
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
