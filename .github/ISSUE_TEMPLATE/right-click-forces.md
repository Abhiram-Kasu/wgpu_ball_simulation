---
name: Right-Click to Apply Forces
about: Add functionality to apply forces to balls using right-click
title: 'Feature: Allow right-click to apply forces to balls'
labels: enhancement, physics, user-interaction
assignees: ''
---

## Feature Description
Add the ability for users to apply directional forces to nearby balls by right-clicking on the simulation window.

## Proposed Behavior
- **Right Click**: Apply a repulsive or attractive force to balls near the cursor
- **Drag (Optional)**: Hold right-click and drag to continuously apply forces
- Force characteristics:
  - Radial effect from cursor position
  - Strength inversely proportional to distance
  - Configurable force magnitude and radius

## Implementation Considerations
- [ ] Capture right mouse button events
- [ ] Calculate cursor position in simulation coordinates
- [ ] Determine which balls are within force radius
- [ ] Apply force vectors to affected balls' velocities
- [ ] Implement force calculation (e.g., inverse square law)
- [ ] Add configuration for force strength and radius
- [ ] Consider adding visual feedback (force radius indicator)

## Force Options
Could support multiple force modes:
1. **Repulsive**: Push balls away from cursor (default)
2. **Attractive**: Pull balls toward cursor (with modifier key)
3. **Directional**: Apply force in drag direction
4. **Vortex**: Create swirling motion around cursor

## User Experience
- Intuitive interaction: right-click to influence balls
- Visual feedback showing force radius and strength
- Smooth force application without causing simulation instability
- Toggle force mode via keyboard shortcut (e.g., Shift for attraction)

## Technical Notes
- Mouse button state tracking via `winit::event::MouseInput`
- Cursor position updates from `winit::event::WindowEvent::CursorMoved`
- Force application in compute shader or CPU-side before buffer update
- May need to pass cursor position and force parameters to GPU
- Consider performance impact when many balls are affected

## Configuration Parameters
Suggested new config options:
```rust
pub struct ForceConfig {
    pub force_magnitude: f32,  // Base force strength
    pub force_radius: f32,     // Effect radius in pixels
    pub force_mode: ForceMode, // Repulsive, Attractive, etc.
}
```

## Priority
Medium - Adds significant interactivity and fun factor

## Related Features
This feature complements:
- Click to drop balls (#1)
- GUI for parameter editing (#4)
