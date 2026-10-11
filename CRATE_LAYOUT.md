# Crate Layout

## Level 0

The bottom-most low-level(ish) parts of the engine/game. They should not have preludes, and (with the exception of starbloom-base and starbloom-derive, which nearly everything will depend on) depend on each other.

### Shared

- starbloom-base -> Plugin trait, Position component, StateSet trait
- starbloom-states -> ServerTickStep and ClientTickStep system sets
- starbloom-derive(proc-macro) -> All macros. SHOULD NOT DEPEND ON ANYTHING ELSE TO REDUCE COMPILE COST

### Client

- starbloom-renderer -> Renderer resource, FrameData resource
- starbloom-bootstrap -> Texture/shader/audio loading utils

## Level 0.5

The multiplayer-seperation layer. Everything above this layer should be client/server agnostic (including UIs!), though with the client/server specific archetecture getting stripped out depending on the feature flags.

- starbloom-