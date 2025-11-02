struct Circle {
    position: vec2<f32>,
    radius: f32,
    _padding1: f32,
    color: vec4<f32>,
    velocity: vec2<f32>,
    _padding2: vec2<f32>,
}

@group(0) @binding(0) var<storage, read_write> input_circles: array<Circle>;

struct Params {
    gravity: vec2<f32>,
    epsilon: f32,
    restitution: f32,
    damping: f32,
    max_velocity: f32,
    dt: f32,
    collision_softness: f32,
}

@group(0) @binding(1) var<uniform> params: Params;

struct ScreenDimensions {
    width: f32,
    height: f32,
}

@group(0) @binding(2) var<uniform> screen_dims: ScreenDimensions;

@compute
@workgroup_size(128)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let index = gid.x;

    // Bounds check
    if index >= arrayLength(&input_circles) {
        return;
    }


    //Apply gravity (acceleration)

    input_circles[index].velocity += params.gravity * params.dt;


    //Apply damping (air resistance/friction)

    input_circles[index].velocity *= params.damping;


    //Update position based on velocity (integration)

    input_circles[index].position += input_circles[index].velocity * params.dt;


    //Resolve collisions with other balls

    for (var i: u32 = 0u; i < arrayLength(&input_circles); i++) {
        if i == index {
            continue;
        }

        let other = input_circles[i];
        let delta = other.position - input_circles[index].position;
        let distance = length(delta);
        let min_distance = input_circles[index].radius + other.radius;

        // Check for collision
        if distance < min_distance && distance > 0.0001 {
            // Normalized direction from this ball to other ball
            let dir = delta / distance;
            let overlap = min_distance - distance;


            // Position Correction (soft separation)

            // Use collision_softness to control how much balls can compress
            // 0.5 = each ball moves half the overlap
            // Lower values = harder collisions (less compression allowed)
            let separation = overlap * params.collision_softness;
            input_circles[index].position -= separation * dir;


            // Velocity-based Collision Response

            // Calculate relative velocity along collision normal
            let relative_velocity = input_circles[index].velocity - other.velocity;
            let velocity_along_normal = dot(relative_velocity, dir);

            // Only apply impulse if circles are moving towards each other
            if velocity_along_normal < 0.0 {
                // Impulse calculation (one-sided for GPU safety)
                // Using coefficient of restitution for energy loss
                let impulse_strength = -(1.0 + params.restitution) * velocity_along_normal * 0.5;

                // Apply impulse to velocity
                input_circles[index].velocity -= impulse_strength * dir;
            }
        }
    }


    //Clamp velocity to prevent extreme speeds

    let speed = length(input_circles[index].velocity);
    if speed > params.max_velocity {
        input_circles[index].velocity = (input_circles[index].velocity / speed) * params.max_velocity;
    }

    //
    //Handle boundary collisions

    let radius = input_circles[index].radius;

    // Left boundary
    if input_circles[index].position.x - radius < 0.0 {
        input_circles[index].position.x = radius;
        input_circles[index].velocity.x = abs(input_circles[index].velocity.x) * params.restitution;
    }

    // Right boundary
    if input_circles[index].position.x + radius > screen_dims.width {
        input_circles[index].position.x = screen_dims.width - radius;
        input_circles[index].velocity.x = -abs(input_circles[index].velocity.x) * params.restitution;
    }

    // Top boundary
    if input_circles[index].position.y - radius < 0.0 {
        input_circles[index].position.y = radius;
        input_circles[index].velocity.y = abs(input_circles[index].velocity.y) * params.restitution;
    }

    // Bottom boundary
    if input_circles[index].position.y + radius > screen_dims.height {
        input_circles[index].position.y = screen_dims.height - radius;
        input_circles[index].velocity.y = -abs(input_circles[index].velocity.y) * params.restitution;
    }
}
