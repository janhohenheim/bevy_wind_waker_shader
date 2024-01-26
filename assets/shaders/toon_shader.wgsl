#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings as view_bindings,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct ToonShaderConfig {
    highlight_color: vec4<f32>,
    shadow_color: vec4<f32>,
    rim_color: vec4<f32>,
}

@group(1) @binding(100) var mask: texture_2d<f32>;
@group(1) @binding(101) var mask_sampler: sampler;
@group(1) @binding(102) var<uniform> toon: ToonShaderConfig;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // we can optionally modify the input before lighting and alpha_discard is applied
    // remove texture
    let texture = pbr_input.material.base_color;
    pbr_input.material.base_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // we can optionally modify the lit color before post-processing is applied
    // Source for cel shading: https://www.youtube.com/watch?v=mnxs6CR6Zrk]
    // sample mask at the current fragment's intensity as u to get the cutoff
    let uv = vec2<f32>(out.color.r, 0.0);
    let mask = textureSample(mask, mask_sampler, uv);
    out.color = out.color * mask;

    // apply toon tone (values taken from palette_reference.png)
    let light_tone = vec4<f32>(254.0, 254.0, 254.0, 255.0) / 255.0;
    let dark_tone = vec4<f32>(163.0, 152.0, 146.0, 255.0) / 255.0;
    out.color = mix(toon.shadow_color, toon.highlight_color, out.color);

    // apply rim highlights. Inspired by Breath of the Wild. (https://www.youtube.com/watch?v=By7qcgaqGI4)
    let eye = normalize(view_bindings::view.world_position.xyz - in.world_position.xyz);
    let rim = 1.0 - dot(eye, in.world_normal);
    let rim_factor = rim * rim;
    out.color = mix(out.color, toon.rim_color, rim_factor);

    // Reapply texture
    out.color = out.color * texture;

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
#endif

    return out;
}
