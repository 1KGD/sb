# Todo

- [x] `egor` port, so that wasm builds work
  - [x] A render function queue, that takes the framecontext as an arg. Due to egor and bevy_ecs being mutually antagonistic of each other.
  - [ ] Plenty of rendering utilities in `starbloom-base` (potentially another crate, `starbloom-rendering`?)
    - [ ] Image rendering
    - [ ] Spritesheet (animated) rendering
    - [ ] Ui rendering (almost certanly in `starbloom-ui`)
      - [ ] Custom, distinct `egui` theme (old windows?).
- [ ] Mobile support (`starbloom-mobile`)
    - [ ] `egor` lacks multitouch support?
    - [ ] Virtual joystick, buttons
  - [ ] Native mobile (android, almost certainly no IOS)
  - [ ] Web mobile (with `gilrs`)
- [ ] Controller support using `gilrs`
  - [ ] Integrate into input abstraction utils in `starbloom-base`
  - [ ] Leave out of native mobile builds, as `gilrs` does not support it
