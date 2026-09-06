# STARBLOOM
(rust rewrite)

>[!WARNING]
>This project is **very early in development**. Nothing is fully fleshed out yet (including the rendering backend). I might also just randomly drop the project without warning (I tend to do that).

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

### Premise
The end goal for this game is "A factory building farming game". For a full breakdown, please see the ["About" docs page](https://sb.readthedocs.io/latest/about.html), which is the current impromptu design doc (the one outside of my head, at least) of what this project is currently intended to turn into.

# Installing

### Prebuilt Files
[![Release](https://github.com/1KGD/sb/actions/workflows/release.yml/badge.svg)](https://github.com/1KGD/sb/actions/workflows/release.yml)

Prebuilt files for windows, macos, and linux (x86 only; Sorry, arm) can be grabbed from the [github releases](https://github.com/1KGD/sb/releases). These should work out-of-the-box.

### Cargo
To install the executable "starbloom" command, first [install rustup](https://rust-lang.org/learn/get-started). Once you have done that, it is as easy as:

```bash
cargo install starbloom
```

Then, run the `starbloom` command to launch the game.

### Build-it-yourself
[TODO: get builds working in the first place]

# Versions
This project tries to follow SemVer to the best of it's ablities:
- `MAJOR (X.0.0)` releases are for big, named updates that add major new systems and content to the game.
  - v1.0.0 should be a fully-playable (though absolutely not feature-complete) game, with a full "story" and gameloop.
- `MINOR (x.Y.0)` releases are for small gameplay features or changes. QOL stuff (unless it coencides with a `MAJOR` release) typically falls into this.
- `PATCH (x.y.Z)` releases are bugfixes and minor tweaks, with little ("returns to working as intended") to no impact on gameplay.
