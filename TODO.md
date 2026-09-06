# Todo

- [x] `egor` port, so that wasm builds work
  - [x] A render function queue, that takes the framecontext as an arg. Due to egor and bevy_ecs being mutually antagonistic of each other.
  - [ ] Plenty of rendering utilities in `starbloom-base` (potentially another crate, `starbloom-rendering`?)
    - [ ] Image rendering
    - [ ] Spritesheet (animated) rendering
    - [ ] Ui rendering (almost certanly in `starbloom-ui`)
      - [ ] `egui`? (`egor` has built-in support). Or a custom UI implementation (more work, but also more stylistic control).
- [ ] Mobile support
    - [ ] `egor` lacks multitouch support?
    - [ ] Virtual joystick, buttons
  - [ ] Native mobile (android, almost certainly no IOS)
  - [ ] Web mobile
- [ ] Controller support
  - [ ] No native controller support for `egor`, find useful library (what does `bevy` use?)
