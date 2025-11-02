struct Circle {
    position: vec2<f32>,
    radius: f32,
    _padding1: f32,
    color: vec4<f32>,
    velocity: vec2<f32>,
    _padding2: vec2<f32>,
}

struct ScreenDimensions {
    width: f32,
    height: f32,
}

@group(0) @binding(0) var<storage, read> circles: array<Circle>;
@group(0) @binding(1) var<uniform> screen_dims: ScreenDimensions;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) speedMixer: f32
}

@vertex
fn vertex_main(
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32
) -> VertexOutput {
    var output: VertexOutput;

    // Get circle data
    let circle = circles[instance_index];

    // Create a quad using 6 vertices (2 triangles)
    // Vertex positions for a quad from -1 to 1
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),  // bottom-left
        vec2<f32>(1.0, -1.0),   // bottom-right
        vec2<f32>(1.0, 1.0),    // top-right
        vec2<f32>(-1.0, -1.0),  // bottom-left
        vec2<f32>(1.0, 1.0),    // top-right
        vec2<f32>(-1.0, 1.0)    // top-left
    );

    let quad_pos = positions[vertex_index];

    // Scale by radius in pixel space
    let pixel_pos = quad_pos * circle.radius + circle.position;

    // Convert pixel coordinates to NDC
    // Pixel (0, 0) is top-left, NDC (-1, -1) is bottom-left
    // x: [0, width] -> [-1, 1]
    // y: [0, height] -> [1, -1] (flip y-axis)
    let ndc_x = (pixel_pos.x / screen_dims.width) * 2.0 - 1.0;
    let ndc_y = 1.0 - (pixel_pos.y / screen_dims.height) * 2.0;

    output.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    output.color = circle.color;
    output.uv = quad_pos;
    output.speedMixer = length(circle.velocity) / /* max velocity */ 30.0;


    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate distance from center of the quad
    let dist = length(input.uv);

    // Discard fragments outside the circle
    if dist > 1.0 {
        discard;
    }

    // Anti-aliasing at the edge
    let edge_smoothness = 0.02;
    let alpha = 1.0 - smoothstep(1.0 - edge_smoothness, 1.0, dist);

    let inverted_color = vec3<f32>(1.0) - input.color.rgb;
    let color = mix(input.color.rgb, inverted_color, input.speedMixer);

    return vec4<f32>(color, input.color.a * alpha);
}
