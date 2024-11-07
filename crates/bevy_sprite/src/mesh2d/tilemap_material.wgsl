// #import bevy_sprite::{
//     mesh2d_vertex_output::VertexOutput,
//     mesh2d_view_bindings::view,
// }

// #ifdef TONEMAP_IN_SHADER
// #import bevy_core_pipeline::tonemapping
// #endif

struct TilemapMaterial {

};

@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = textureSample(texture, texture_sampler, mesh.uv);

// #ifdef VERTEX_COLORS
//     output_color = output_color * mesh.color;
// #endif

// #ifdef TONEMAP_IN_SHADER
//     output_color = tonemapping::tone_mapping(output_color, view.color_grading);
// #endif
    return output_color;
}

fn alpha_discard(material: ColorMaterial, output_color: vec4<f32>) -> vec4<f32> {
    var color = output_color;
    let alpha_mode = material.flags & COLOR_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS;
    if alpha_mode == COLOR_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE {
        // NOTE: If rendering as opaque, alpha should be ignored so set to 1.0
        color.a = 1.0;
    }


    return color;
}