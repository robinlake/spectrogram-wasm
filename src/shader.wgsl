// Vertex shader

struct VertexOutput {
    // this is the value we want to use as the vertex's clip coordinates. This is analogous to GLSL's gl_Position variable
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

// Fragment shader
@fragment
// The @location(0) bit tells WGPU to store the vec4 value returned by this function in the first color target
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
 