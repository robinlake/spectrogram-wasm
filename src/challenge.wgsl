// Vertex shader

// Vertex shader

struct VertexInput {
    @location(0) position: vec4<f32>,
    // @location(1) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

// struct VertexInput {
//     @location(0) position: vec3<f32>,
//     @location(1) color: vec3<f32>,
//     // @location(1) tex_coords: vec2<f32>,
// };


// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) position: vec2<f32>,
// };

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    // @location(0) tex_coords: vec2<f32>,

};

// @vertex
// fn vs_main(
//     @builtin(vertex_index) in_vertex_index: u32,
// ) -> VertexOutput {
//     var out: VertexOutput;
//     let x = f32(1 - i32(in_vertex_index)) * 0.5;
//     let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
//     out.position = vec2<f32>(x, y);
//     out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
//     return out;
// }


// @vertex
// fn vs_main(
//     model: VertexInput,
// ) -> VertexOutput {
//     var out: VertexOutput;
//     out.tex_coords = model.tex_coords;
//     out.clip_position = vec4<f32>(model.position, 1.0);
//     return out;
// }

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

// @group(0) @binding(0)
// var t_diffuse: texture_2d<f32>;
// @group(0)@binding(1)
// var s_diffuse: sampler;

@fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//     return vec4<f32>(in.position, 0.5, 1.0);
// }
// @fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//     return textureSample(t_diffuse, s_diffuse, in.tex_coords);
// }

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}







