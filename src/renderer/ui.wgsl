//Used for rendering rectangles
//Not done

struct UIVertexInput {
    @location(0) uv: vec2<f32>, 
    @location(1) color: vec4<f32>,
    @location(2) pos: vec3<f32>,
};

struct UIVertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vertex(in: UIVertexInput) -> UIVertexOutput {
    var out: UIVertexOutput;
    out.uv = in.uv;
    out.color = in.color;
    //wgpu shaders use -1 to 1
    out.position = vec4(in.pos.x, in.pos.y, in.pos.z, 1.0);
    return out;
}

@fragment
fn fragment(vertex: UIVertexOutput) -> @location(0) vec4<f32> {
    return vertex.color;
}