# STARBLOOM
(rust rewrite)

>[!WARNING]
>This project is **very early in development**. Nothing is fully fleshed out yet (including the rendering backend). I might also just randomly drop the project without warning (I tend to do that).

### Premise
The end goal for this game is "A factory building farming game". For a full breakdown, please see the ["About" docs page](https://sb.readthedocs.io/latest/about.html), which is the current impromptu design doc (the one outside of my head, at least) of what this project is currently intended to turn into.

# Installing

### Cargo
To install the executable "starbloom" command, first [install rustup](https://rust-lang.org/learn/get-started). Once you have done that, it is as easy as:

```bash
cargo install starbloom
```

Then, run the `starbloom` command to launch the game.

# Versions
This project tries to follow SemVer to the best of it's ablities:
- `MAJOR (X.0.0)` releases are for big, named updates that add major new systems and content to the game.
  - v1.0.0 should be a fully-playable (though absolutely not feature-complete) game, with a full "story" and gameloop.
- `MINOR (x.Y.0)` releases are for small gameplay features or changes. QOL stuff (unless it coencides with a `MAJOR` release) typically falls into this.
- `PATCH (x.y.Z)` releases are bugfixes and minor tweaks, with little ("fixes a crash") to no impact on gameplay.
