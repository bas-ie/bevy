// #import bevy_sprite::{
//     mesh2d_vertex_output::VertexOutput,
//     mesh2d_view_bindings::view,
// }

// #ifdef TONEMAP_IN_SHADER
// #import bevy_core_pipeline::tonemapping
// #endif

struct TilemapMaterial {

};

@group(2) @binding(0) var indicies: texture_2d<u32>;
@group(2) @binding(1) var indicies_sampler: sampler;
@group(2) @binding(2) var texture: texture_2d<f32>;
@group(2) @binding(3) var texture_sampler: sampler;

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
