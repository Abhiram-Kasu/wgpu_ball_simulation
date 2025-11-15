---
name: Add Blur Effect for Fluid Simulation
about: Implement motion blur or particle blur to create a more fluid-like appearance
title: 'Feature: Add blur effect to make simulation appear more like a fluid'
labels: enhancement, rendering, visual-effects
assignees: ''
---

## Feature Description
Add a blur or smoothing effect to the ball rendering to create a more fluid-like or smooth particle appearance, making the simulation visually resemble fluid dynamics.

## Proposed Visual Effects

### Option 1: Motion Blur
- Blur balls based on their velocity
- Faster-moving balls have longer blur trails
- Creates sense of motion and fluidity

### Option 2: Gaussian Blur Post-Processing
- Apply full-screen Gaussian blur as a post-processing effect
- Adjustable blur radius/strength
- Smooth, cohesive fluid-like appearance

### Option 3: Additive Blending with Trails
- Render ball positions over multiple frames with fading
- Creates persistent trails showing motion history
- Combined with blur creates fluid flow visualization

### Option 4: Metaball Rendering
- Render balls with additive blending
- Apply threshold filter to create smooth merged surfaces
- Most authentic fluid-like appearance

## Implementation Considerations
- [ ] Choose blur technique(s) to implement
- [ ] Add intermediate render target(s) for post-processing
- [ ] Implement blur shader (Gaussian, box blur, or other)
- [ ] Configure blur parameters (radius, strength, samples)
- [ ] Optimize for performance (separable blur passes)
- [ ] Add toggle to enable/disable blur effect
- [ ] Consider temporal accumulation for trails

## Rendering Pipeline Changes

### For Post-Processing Blur:
1. Render balls to intermediate texture
2. Apply horizontal blur pass
3. Apply vertical blur pass (separable Gaussian)
4. Present final blurred result

### For Additive/Metaball:
1. Change blend mode to additive
2. Render with gradient falloff from ball centers
3. Apply threshold or posterization filter
4. Add color grading for fluid-like appearance

## Configuration Parameters
```rust
pub struct BlurConfig {
    pub enabled: bool,
    pub blur_radius: f32,      // Blur kernel size
    pub blur_strength: f32,    // Blur intensity (0.0-1.0)
    pub motion_blur: bool,     // Velocity-based blur
    pub trail_length: f32,     // For motion trails
}
```

## User Experience
- Toggle blur on/off with keyboard shortcut (e.g., 'L' for blur)
- Adjust blur parameters in real-time
- Significant visual transformation from discrete balls to fluid-like substance
- Performance should remain acceptable (>30 FPS)

## Technical Notes
- Post-processing requires additional render passes
- Separable Gaussian blur is most efficient (2 passes vs N² samples)
- May need ping-pong buffers for multi-pass effects
- Consider using compute shaders for blur instead of fragment shaders
- Texture format should support blending (RGBA8, RGBA16F)
- Viewport-sized textures needed for full-screen effects

## Performance Considerations
- Blur increases GPU workload significantly
- Provide quality presets (Low/Medium/High)
- Consider reduced resolution render targets for blur
- Profile frame time before/after implementation

## Visual References
- Smooth Particle Hydrodynamics (SPH) visualizations
- Metaball fluid rendering
- Particle system motion blur in game engines

## Priority
Low-Medium - Visual enhancement, not core functionality

## Related Features
- GUI for toggling and adjusting blur parameters (#4)
- Could enhance the visual impact of force application (#2)
