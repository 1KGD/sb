use std::collections::hash_map::HashMap;

use bevy_ecs::prelude::*;
use egor::input::*;

use crate::actions::*;

#[derive(Default, Resource)]
pub struct InputCtx {
    key_states: HashMap<KeyCode, bool>,
}

impl InputCtx {
    pub fn action_held(&self, action: Action) -> bool {
        for key in key_mappings_from_action(action).unwrap_or(&vec![]) {
            if *self.key_states.get(&key).unwrap_or(&false) {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, input: &Input) {
        for key in all_key_codes() {
            self.key_states.insert(key, input.key_held(key));
        }
    }
}
