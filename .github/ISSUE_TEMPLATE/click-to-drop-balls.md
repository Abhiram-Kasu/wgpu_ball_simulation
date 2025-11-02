---
name: Click to Drop More Balls
about: Add functionality to drop balls by clicking on the simulation window
title: 'Feature: Allow click to drop more balls into the simulation'
labels: enhancement, user-interaction
assignees: ''
---

## Feature Description
Add the ability for users to click anywhere on the simulation window to spawn new balls at the cursor position.

## Proposed Behavior
- **Left Click**: Drop a single ball at the cursor position
- The new ball should:
  - Appear at the exact click coordinates
  - Have the same radius as configured balls
  - Have random or default color (matching current configuration)
  - Start with minimal/zero initial velocity
  - Immediately participate in physics simulation

## Implementation Considerations
- [ ] Capture mouse click events in the event loop
- [ ] Convert window coordinates to simulation space coordinates
- [ ] Dynamically add new `Circle` instances to the simulation
- [ ] Update GPU buffers to include new balls
- [ ] Consider maximum ball count limits for performance
- [ ] Handle buffer reallocation if needed

## User Experience
- Users can interactively add balls during simulation runtime
- Provides experimentation capability without restarting the program
- Enhances the interactive nature of the simulation

## Technical Notes
- Mouse click events can be captured through `winit::event::WindowEvent::MouseInput`
- Cursor position available via `winit::event::WindowEvent::CursorMoved`
- May require dynamic buffer resizing or pre-allocated buffer pools
- Consider adding visual feedback (cursor highlight, particle spawn effect)

## Priority
Medium - Enhances user interaction but not critical for core simulation

## Related Features
This feature could work well with:
- Right-click force application (#2)
- GUI parameter editing (#4)
