# Todo

- [ ] `egor` port, so that wasm builds work
  - [ ] A render function queue, that takes the framecontext as an arg. Due to egor and bevy_ecs being mutually antagonistic of each other.
  - [ ] Plenty of rendering utilities in `starbloom-base` (potentially another crate, `starbloom-rendering`?)
    - [ ] Image rendering
    - [ ] Spritesheet (animated) rendering
    - [ ] Ui rendering (almost certanly in `starbloom-ui`)
      - [ ] `egui`? (`egor` has built-in support). Or a custom UI implementation (more work, but also more stylistic control).
