---
name: Add GUI for Parameter Editing
about: Implement an on-screen GUI for viewing and editing simulation parameters
title: 'Feature: Add GUI to display and edit parameters on-screen'
labels: enhancement, ui, user-experience
assignees: ''
---

## Feature Description
Add an on-screen graphical user interface (GUI) that displays current simulation parameters and allows real-time editing without using keyboard shortcuts.

## Proposed Features

### Parameter Display Panel
Show current values for all simulation parameters:
- **Physics**: Gravity (X/Y), Damping, Restitution, Max Velocity
- **Simulation**: Timestep (dt), Collision Softness
- **Scene**: Ball Count, Ball Radius, FPS counter
- **Optional**: Force parameters, blur settings (if implemented)

### Interactive Controls
- **Sliders**: For continuous parameters (gravity, damping, etc.)
- **Number Inputs**: For precise value entry
- **Checkboxes**: For boolean toggles (blur on/off, etc.)
- **Buttons**: Reset to defaults, spawn balls, etc.
- **Color Pickers**: For ball colors or background

### Layout Options
1. **Side Panel**: Fixed panel on left or right side
2. **Overlay**: Semi-transparent floating window
3. **Collapsible**: Can be hidden/shown with hotkey (Tab/F1)

## Recommended GUI Libraries

### Option 1: egui (Recommended)
- **Pros**: Immediate mode, easy integration, good WGPU support
- **Cons**: Adds dependency
- **Integration**: `egui-wgpu` and `egui-winit` crates

### Option 2: imgui-rs
- **Pros**: Mature, widely used, feature-rich
- **Cons**: Requires imgui-wgpu-rs, more complex setup

### Option 3: Custom ImGUI-style
- **Pros**: No dependencies, full control
- **Cons**: Time-consuming, reinventing the wheel

## Implementation Considerations
- [ ] Choose and integrate GUI library (recommend egui)
- [ ] Set up GUI rendering pipeline
- [ ] Create parameter control widgets
- [ ] Bind widgets to simulation parameters
- [ ] Handle real-time parameter updates
- [ ] Add FPS counter and performance metrics
- [ ] Implement GUI toggle key
- [ ] Style GUI to match simulation aesthetic
- [ ] Handle mouse events for GUI interaction
- [ ] Prevent GUI clicks from affecting simulation

## Example Layout
```
┌─────────────────────────────┐
│  Ball Simulation Controls   │
├─────────────────────────────┤
│ FPS: 60.2                   │
│ Balls: 20000                │
├─────────────────────────────┤
│ Gravity                     │
│   X: [====|----] 0.0        │
│   Y: [=======|--] 10.0      │
│                             │
│ Physics                     │
│   Damping: [======|-] 0.99  │
│   Restitution: [==|--] 0.5  │
│   Max Vel: [====|---] 30.0  │
│                             │
│ Simulation                  │
│   dt: [===|-----] 0.016     │
│   Softness: [==|--] 0.5     │
│                             │
│ [Reset] [Spawn Balls]       │
└─────────────────────────────┘
```

## User Experience
- Intuitive visual feedback for all parameters
- Real-time updates while dragging sliders
- Tooltips explaining each parameter
- Responsive to window resizing
- Non-intrusive: can be hidden during simulation
- Keyboard shortcuts still work as fallback

## Technical Notes
- GUI rendering happens after simulation, before present
- Need separate render pass for GUI overlay
- Mouse events need routing to GUI system
- Parameter changes must update GPU buffers immediately
- Consider state synchronization between GUI and simulation
- Profile performance impact of GUI rendering

## Integration Example (egui)
```rust
// In Cargo.toml
egui = "0.29"
egui-wgpu = "0.29"
egui-winit = "0.29"

// In app.rs
let egui_ctx = egui::Context::default();
let egui_renderer = egui_wgpu::Renderer::new(...);
let egui_state = egui_winit::State::new(...);
```

## Suggested Dependencies
```toml
[dependencies]
egui = "0.29"
egui-wgpu = "0.29"
egui-winit = "0.29"
```

## Priority
High - Significantly improves usability and discoverability of features

## Related Features
This GUI should include controls for:
- Current keyboard-adjustable parameters
- Click-to-spawn ball feature (#1)
- Force application parameters (#2)
- Blur effect settings (#3)

## Accessibility
- Support keyboard navigation
- Clear labels and units for all parameters
- Consider color-blind friendly palette
- Adequate contrast for readability
