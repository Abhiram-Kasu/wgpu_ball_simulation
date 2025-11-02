# WGPU Ball Simulation

A high-performance 2D ball physics simulation built with Rust and WGPU, featuring real-time GPU-accelerated collision detection and physics calculations.

## Features

- **GPU-Accelerated Physics**: Leverages WGPU compute shaders for efficient simulation of thousands of balls
- **Real-time Interactive Controls**: Adjust simulation parameters on the fly using keyboard controls
- **Customizable Physics**: Fine-tune gravity, damping, restitution, collision softness, and more
- **High Performance**: Handles thousands of balls simultaneously with smooth 60 FPS rendering

## Prerequisites

- Rust (edition 2024 or later)
- A GPU with support for WGPU backends (Vulkan, Metal, DX12, or WebGPU)

## Installation

```bash
git clone https://github.com/Abhiram-Kasu/wgpu_ball_simulation.git
cd wgpu_ball_simulation
cargo build --release
```

## Running the Simulation

```bash
cargo run --release
```

## Keyboard Controls

The simulation provides extensive keyboard controls for real-time parameter adjustment:

### Gravity
- **Arrow Keys**: Adjust gravity direction
  - ↑/↓: Increase/Decrease vertical gravity
  - ←/→: Increase/Decrease horizontal gravity
- **Space**: Reset gravity to zero

### Physics Parameters
- **D/F**: Decrease/Increase damping (air resistance)
- **R/T**: Decrease/Increase restitution (bounciness)
- **V/B**: Decrease/Increase max velocity cap
- **C/X**: Decrease/Increase collision softness
- **Z/A**: Decrease/Increase timestep (simulation speed)

### Help
- **H**: Display full controls and current parameter values

## Configuration

You can customize the simulation by modifying the `SimulationConfig` in `src/main.rs`:

```rust
let config = SimulationConfig {
    window_width: 1600,
    window_height: 900,
    ball_count: 20000,
    ball_radius: 5.0,
    ball_color: Some([0.0, 1.0, 1.0, 0.5]), // Or None for random colors
    gravity: [0.0, 10.0],
    restitution: 0.5,
    damping: 0.99,
    max_velocity: 30.0,
    dt: 0.016,               // ~60 FPS timestep
    collision_softness: 0.5, // 0.0 = hard, 1.0 = very soft
};
```

## Project Structure

```
wgpu_ball_simulation/
├── src/
│   ├── main.rs           # Entry point and configuration
│   ├── app.rs            # Application logic and event handling
│   └── shaders/
│       ├── compute_circle.wgsl  # Physics compute shader
│       └── draw_circles.wgsl    # Rendering shader
├── Cargo.toml
└── README.md
```

## How It Works

1. **Initialization**: Balls are generated in a grid pattern with random or specified colors
2. **Compute Pass**: GPU compute shaders calculate physics (collisions, boundaries, gravity) for all balls in parallel
3. **Render Pass**: Balls are rendered as instanced quads with fragment shader circle rendering
4. **Continuous Loop**: The simulation runs at ~60 FPS with configurable timestep

## Dependencies

- `wgpu`: GPU abstraction layer for compute and rendering
- `winit`: Cross-platform window creation and event handling
- `bytemuck`: Safe casting between plain data types
- `cgmath`: Linear algebra for vector operations
- `rand`: Random number generation for ball initialization
- `pollster`: Blocking executor for async operations

## Performance Tips

- Reduce `ball_count` for lower-end GPUs
- Increase `dt` for faster simulation (may reduce stability)
- Adjust `collision_softness` if balls are passing through each other
- Use `--release` flag for optimal performance

## License

This project is open source and available for use and modification.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
