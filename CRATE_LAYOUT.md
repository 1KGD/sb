# Crate Layout

## Level 0

The bottom-most low-level(ish) parts of the engine/game. They should not have preludes, and (with the exception of starbloom-base and starbloom-derive, which nearly everything will depend on) depend on each other.

### Shared

- `starbloom-base` -> Plugin trait, Position component, StateSet trait
- `starbloom-states` -> ServerTickStep and ClientTickStep system sets
- `starbloom-derive`*(proc-macro)* -> All macros. SHOULD NOT DEPEND ON ANYTHING ELSE TO REDUCE COMPILE COST

### Client

- `starbloom-renderer` -> Renderer resource, FrameData resource
- `starbloom-bootstrap` -> Texture/shader/audio loading utils
- `starbloom-input` -> Keyboard, touch, & `gilrs` input abstraction and mapping.

### Server

- `starbloom-worldgen` -> Map loading/generating

## Level 0.5

The multiplayer-seperation layer. Everything above this layer should be client/server agnostic (including UIs!), though with the client/server specific archetecture getting stripped out depending on the feature flags.

- `starbloom-core-shared` -> Re-exports all of the stuff in the Shared section of Level 0.
- `starbloom-core-client` -> Re-exports all of the stuff in the Client section of Level 0. If the feature flags indicate a dedicated server, then this crate will export *NOTHING*, but can still be depended on. This is to reduce conditional `use` boilerplate, but also prevents accedentally using client stuff in the dedicated server.
- `starbloom-core-server` -> Re-exports all of the stuff in the Server section of Level 0. If the feature flags indicate a dedicated client (no singleplayer functionality), it will do the same no-export thing as `starbloom-core-client`.

## Level 1

- `starbloom-intro` -> Intro sequence. Completely ignored if the `intro` feature flag is omitted.
