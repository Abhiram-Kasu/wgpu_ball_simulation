use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    wgt::{CommandEncoderDescriptor, DeviceDescriptor},
    *,
};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
    *,
};

use rand::Rng;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Circle {
    position: [f32; 2],
    radius: f32,
    _padding1: f32,
    color: [f32; 4],
    velocity: [f32; 2],
    _padding2: [f32; 2],
}

impl Circle {
    pub fn new(position: [f32; 2], radius: f32, color: [f32; 4], velocity: [f32; 2]) -> Self {
        Self {
            position,
            radius,
            _padding1: 0.0,
            color,
            velocity,
            _padding2: [0.0; 2],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct ScreenDimensions {
    width: f32,
    height: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Params {
    /*
    *
    struct Params {
        gravity: vec2<f32>,
        epsilon: f32,
        restitution: f32,
        damping: f32,
        max_velocity: f32,
        dt: f32,
        collision_softness: f32,
    }

    */
    gravity: [f32; 2],
    epsilon: f32,
    restitution: f32,
    damping: f32,
    max_velocity: f32,
    dt: f32,
    collision_softness: f32,
}

pub struct AppState {
    window: Arc<Window>,

    instance: Instance,
    surface: Surface<'static>,
    adapter: Adapter,
    device: Device,
    queue: Queue,

    compute_pipeline: ComputePipeline,
    compute_bind_group: BindGroup,

    circle_buffer: Buffer,
    circles: Vec<Circle>,
    params: Params,
    params_buffer: Buffer,

    screen_dimensions_buffer: Buffer,

    render_pipeline: RenderPipeline,
    render_bind_group: BindGroup,

    render_pipeline_layout: PipelineLayout,
}

impl AppState {
    pub async fn new(window: Arc<Window>, balls: Vec<Circle>, initial_params: Params) -> Self {
        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = unsafe {
            instance
                .create_surface_unsafe(
                    SurfaceTargetUnsafe::from_window(&*window)
                        .expect("Failed to get unsafe ref from window"),
                )
                .expect("Failed to create surface")
        };

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .expect("Failed to create adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            label: Some("Ball Simulation Device"),
            ..Default::default()
        }))
        .expect("Failed to create device");

        let circle_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Balls"),
            contents: bytemuck::cast_slice(&balls),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        });

        let params_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Params Buffer "),
            contents: bytemuck::bytes_of(&initial_params),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let compute_shader =
            device.create_shader_module(include_wgsl!("shaders/compute_circle.wgsl"));

        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Compute Pipeline Descriptor"),
            layout: None,
            module: &compute_shader,
            entry_point: None,
            cache: None,
            compilation_options: Default::default(),
        });

        // Screen dimensions buffer
        let size = window.inner_size();
        let screen_dims = ScreenDimensions {
            width: size.width as f32,
            height: size.height as f32,
        };
        let screen_dimensions_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Screen Dimensions Buffer"),
            contents: bytemuck::bytes_of(&screen_dims),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let compute_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &compute_pipeline.get_bind_group_layout(0),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: circle_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: screen_dimensions_buffer.as_entire_binding(),
                },
            ],
        });

        // Render pipeline setup
        let render_shader = device.create_shader_module(include_wgsl!("shaders/draw_circles.wgsl"));

        let render_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Render Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&render_bind_group_layout],
            push_constant_ranges: &[],
        });

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &render_shader,
                entry_point: Some("vertex_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &render_shader,
                entry_point: Some("fragment_main"),
                targets: &[Some(ColorTargetState {
                    format: surface_format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let render_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Render Bind Group"),
            layout: &render_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: circle_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: screen_dimensions_buffer.as_entire_binding(),
                },
            ],
        });

        surface.configure(
            &device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: size.width,
                height: size.height,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            },
        );

        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            compute_pipeline,
            compute_bind_group,
            circle_buffer,
            circles: balls,
            params: initial_params,
            params_buffer,
            screen_dimensions_buffer,
            render_pipeline,
            render_bind_group,
            render_pipeline_layout,
        }
    }
}

/// Generate a grid of evenly spaced circles with random colors
pub fn generate_random_balls(
    count: usize,
    screen_width: f32,
    screen_height: f32,
    ball_radius: f32,
    color: Option<[f32; 4]>,
) -> Vec<Circle> {
    let mut rng = rand::thread_rng();
    let mut balls = Vec::new();

    // Calculate grid dimensions
    let cols = (count as f32).sqrt().ceil() as usize;
    let rows = (count as f32 / cols as f32).ceil() as usize;

    // Calculate spacing
    let spacing_x = screen_width / (cols as f32 + 1.0);
    let spacing_y = screen_height / (rows as f32 + 1.0);

    let mut created = 0;
    for row in 0..rows {
        for col in 0..cols {
            if created >= count {
                break;
            }

            let x = spacing_x * (col as f32 + 1.0);
            let y = spacing_y * (row as f32 + 1.0);

            // Generate random color
            let color = if let Some(color) = color {
                (color[0], color[1], color[2])
            } else {
                let r: f32 = rng.r#gen();
                let g: f32 = rng.r#gen();
                let b: f32 = rng.r#gen();
                (r, g, b)
            };
            // Very small random velocity for less chaotic starting conditions
            let vx: f32 = rng.gen_range(-0.2..0.2);
            let vy: f32 = rng.gen_range(-0.2..0.2);

            balls.push(Circle {
                position: [x, y],
                radius: ball_radius,
                _padding1: 0.0,
                color: [color.0, color.1, color.2, 1.0],
                velocity: [vx, vy],
                _padding2: [0.0, 0.0],
            });

            created += 1;
        }
        if created >= count {
            break;
        }
    }

    balls
}

pub struct App {
    state: Option<AppState>,
    config: Option<crate::SimulationConfig>,
}

impl App {
    pub fn new(config: crate::SimulationConfig) -> Self {
        Self {
            state: None,
            config: Some(config),
        }
    }

    fn update(&mut self) {
        if let Some(state) = self.state.as_mut() {
            // Run compute pass for simulation
            let mut command_encoder =
                state
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("Command Encoder"),
                    });

            {
                let mut compute_pass = command_encoder.begin_compute_pass(&ComputePassDescriptor {
                    label: Some("Compute pass descriptor"),
                    timestamp_writes: None,
                });

                compute_pass.set_pipeline(&state.compute_pipeline);
                compute_pass.set_bind_group(0, &state.compute_bind_group, &[]);
                let workgroup_count: u32 = state.circles.len().div_ceil(128) as u32;
                compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
            }

            state.queue.submit([command_encoder.finish()]);
        }
    }

    fn render(&mut self) {
        if let Some(state) = self.state.as_mut() {
            // Get current texture
            let output = state
                .surface
                .get_current_texture()
                .expect("Couldn't get texture");

            let view = output.texture.create_view(&Default::default());

            // Create command encoder for rendering
            let mut command_encoder =
                state
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

            {
                let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render pass descriptor"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color::BLACK),
                            store: StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                render_pass.set_pipeline(&state.render_pipeline);
                render_pass.set_bind_group(0, &state.render_bind_group, &[]);
                render_pass.draw(0..6, 0..state.circles.len() as u32);
            }

            state.queue.submit([command_encoder.finish()]);
            output.present();
        }
    }
    fn print_controls_hint() {
        println!("→ Press H for full controls list");
    }

    fn handle_keyboard_input(&mut self, _event: PhysicalKey) {
        if let Some(_state) = self.state.as_mut() {
            let mut params_changed = false;

            match _event {
                // Gravity Controls
                PhysicalKey::Code(KeyCode::ArrowDown) => {
                    _state.params.gravity[1] += 0.1;
                    println!(
                        "Gravity: [{:.2}, {:.2}]",
                        _state.params.gravity[0], _state.params.gravity[1]
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::ArrowUp) => {
                    _state.params.gravity[1] -= 0.1;
                    println!(
                        "Gravity: [{:.2}, {:.2}]",
                        _state.params.gravity[0], _state.params.gravity[1]
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::ArrowLeft) => {
                    _state.params.gravity[0] -= 0.1;
                    println!(
                        "Gravity: [{:.2}, {:.2}]",
                        _state.params.gravity[0], _state.params.gravity[1]
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::ArrowRight) => {
                    _state.params.gravity[0] += 0.1;
                    println!(
                        "Gravity: [{:.2}, {:.2}]",
                        _state.params.gravity[0], _state.params.gravity[1]
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }

                // Damping (Air Resistance) Controls
                PhysicalKey::Code(KeyCode::KeyD) => {
                    _state.params.damping = (_state.params.damping - 0.005).max(0.80);
                    println!(
                        "Damping (air resistance): {:.3} (less damping = more chaotic)",
                        _state.params.damping
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::KeyF) => {
                    _state.params.damping = (_state.params.damping + 0.005).min(0.999);
                    println!(
                        "Damping (air resistance): {:.3} (more damping = slower)",
                        _state.params.damping
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }

                // Restitution (Bounciness) Controls
                PhysicalKey::Code(KeyCode::KeyR) => {
                    _state.params.restitution = (_state.params.restitution - 0.05).max(0.0);
                    println!(
                        "Restitution (bounciness): {:.2} (less bouncy)",
                        _state.params.restitution
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::KeyT) => {
                    _state.params.restitution = (_state.params.restitution + 0.05).min(1.0);
                    println!(
                        "Restitution (bounciness): {:.2} (more bouncy)",
                        _state.params.restitution
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }

                // Max Velocity Controls
                PhysicalKey::Code(KeyCode::KeyV) => {
                    _state.params.max_velocity = (_state.params.max_velocity - 1.0).max(1.0);
                    println!(
                        "Max velocity: {:.1} (slower speed cap)",
                        _state.params.max_velocity
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::KeyB) => {
                    _state.params.max_velocity = (_state.params.max_velocity + 1.0).min(100.0);
                    println!(
                        "Max velocity: {:.1} (faster speed cap)",
                        _state.params.max_velocity
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }

                // Collision Softness Controls (how much balls can compress)
                PhysicalKey::Code(KeyCode::KeyC) => {
                    _state.params.collision_softness =
                        (_state.params.collision_softness - 0.05).max(0.1);
                    println!(
                        "Collision softness: {:.2} (harder collisions)",
                        _state.params.collision_softness
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::KeyX) => {
                    _state.params.collision_softness =
                        (_state.params.collision_softness + 0.05).min(1.0);
                    println!(
                        "Collision softness: {:.2} (softer/more overlap allowed)",
                        _state.params.collision_softness
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }

                // Timestep (dt) Controls
                PhysicalKey::Code(KeyCode::KeyZ) => {
                    _state.params.dt = (_state.params.dt - 0.001).max(0.001);
                    println!(
                        "Timestep (dt): {:.4} (smaller steps = more stable)",
                        _state.params.dt
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::KeyA) => {
                    _state.params.dt = (_state.params.dt + 0.001).min(1.0);
                    println!(
                        "Timestep (dt): {:.4} (larger steps = faster/less stable)",
                        _state.params.dt
                    );
                    Self::print_controls_hint();
                    params_changed = true;
                }

                // Special Actions
                PhysicalKey::Code(KeyCode::Space) => {
                    // Reset to stopped state
                    _state.params.gravity = [0.0, 0.0];
                    println!("Gravity reset to zero");
                    Self::print_controls_hint();
                    params_changed = true;
                }
                PhysicalKey::Code(KeyCode::KeyH) => {
                    // Print help
                    println!("\n=== KEYBOARD CONTROLS ===");
                    println!("Arrow Keys:     Adjust gravity direction");
                    println!("D/F:            Decrease/Increase damping (air resistance)");
                    println!("R/T:            Decrease/Increase restitution (bounciness)");
                    println!("V/B:            Decrease/Increase max velocity");
                    println!("C/X:            Decrease/Increase collision softness");
                    println!("Z/A:            Decrease/Increase timestep (dt)");
                    println!("Space:          Reset gravity to zero");
                    println!("H:              Show this help");
                    println!("\n=== CURRENT PARAMETERS ===");
                    println!(
                        "Gravity: [{:.2}, {:.2}]",
                        _state.params.gravity[0], _state.params.gravity[1]
                    );
                    println!("Damping: {:.3}", _state.params.damping);
                    println!("Restitution: {:.2}", _state.params.restitution);
                    println!("Max velocity: {:.1}", _state.params.max_velocity);
                    println!(
                        "Collision softness: {:.2}",
                        _state.params.collision_softness
                    );
                    println!("Timestep (dt): {:.4}", _state.params.dt);
                    println!("==========================\n");
                }
                _ => {}
            }

            // Update the params buffer on the GPU when params change
            if params_changed {
                _state.queue.write_buffer(
                    &_state.params_buffer,
                    0,
                    bytemuck::bytes_of(&_state.params),
                );
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let config = self.config.as_ref().unwrap();

        let window_attrs = winit::window::Window::default_attributes()
            .with_inner_size(winit::dpi::LogicalSize::new(
                config.window_width,
                config.window_height,
            ))
            .with_title("Ball Simulation");

        let window = Arc::new(
            event_loop
                .create_window(window_attrs)
                .expect("Failed to create window"),
        );

        let size = window.inner_size();
        let balls = generate_random_balls(
            config.ball_count,
            size.width as f32,
            size.height as f32,
            config.ball_radius,
            config.ball_color,
        );

        self.state = Some(pollster::block_on(AppState::new(
            window,
            balls,
            Params {
                gravity: config.gravity,
                epsilon: 0.01,
                restitution: config.restitution,
                damping: config.damping,
                max_velocity: config.max_velocity,
                dt: config.dt,
                collision_softness: config.collision_softness,
            },
        )));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: window::WindowId,
        event: event::WindowEvent,
    ) {
        match event {
            event::WindowEvent::Destroyed | event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            event::WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.handle_keyboard_input(event.physical_key);
            }
            event::WindowEvent::RedrawRequested => {
                self.update();
                self.render();
                if let Some(state) = &self.state {
                    state.window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }
}
