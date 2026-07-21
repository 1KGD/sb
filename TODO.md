# Todo

- [ ] `wgpu` port, so that wasm builds work
  - [ ] Adjust rendering loop (and steal RenderContext non-send resource from the raylib-port branch)
  - [ ] Plenty of rendering utilities in `starbloom-base` (potentially another crate, `starbloom-rendering`?)
    - [ ] Image rendering
    - [ ] Spritesheet (animated) rendering
    - [ ] Ui rendering (almost certanly in `starbloom-ui`)
