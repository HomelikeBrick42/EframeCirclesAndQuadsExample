struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) rectangle_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) rectangle_index: u32,
    @location(1) uv: vec2<f32>,
};

struct Camera {
    position: vec2<f32>,
    aspect: f32,
    zoom: f32,
};

@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Rectangle {
    position: vec2<f32>,
    color: vec3<f32>,
    size: vec2<f32>,
};

@group(1)
@binding(0)
var<storage, read> rectangles: array<Rectangle>;

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.rectangle_index = input.rectangle_index;

    output.uv = vec2<f32>(
        f32((input.vertex_index >> 0u) & 1u) * 2.0 - 1.0,
        f32((input.vertex_index >> 1u) & 1u) * 2.0 - 1.0,
    );

    let world_position = output.uv * rectangles[input.rectangle_index].size * 0.5 + rectangles[input.rectangle_index].position;

    output.clip_position = vec4<f32>((world_position - camera.position) * camera.zoom / vec2<f32>(camera.aspect, 1.0), 0.0, 1.0);

    return output;
}

@fragment
fn pixel(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(rectangles[input.rectangle_index].color, 1.0);
}
