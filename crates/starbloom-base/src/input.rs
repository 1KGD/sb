use std::collections::hash_map::HashMap;

use bevy_ecs::prelude::*;
use egor::input::*;
use log::*;

const TRACKED_KEYS: [KeyCode; 4] = [
    KeyCode::ArrowUp,
    KeyCode::ArrowDown,
    KeyCode::ArrowLeft,
    KeyCode::ArrowRight,
];

#[derive(Default, Resource)]
pub struct InputCtx {
    key_states: HashMap<KeyCode, bool>,
}

impl InputCtx {
    pub fn key_held(&self, key: KeyCode) -> bool {
        *self.key_states.get(&key).unwrap_or(&false)
    }

    pub fn update(&mut self, input: &Input) {
        for key in TRACKED_KEYS {
            self.key_states.insert(key, input.key_held(key));
        }
    }
}
