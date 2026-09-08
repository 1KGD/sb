use egor::input::*;

#[derive(PartialEq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

static ACTION_KEYMAP: std::sync::LazyLock<Vec<(Action, Vec<KeyCode>)>> =
    std::sync::LazyLock::new(|| {
        vec![
            (Action::Up, vec![KeyCode::ArrowUp, KeyCode::KeyW]),
            (Action::Down, vec![KeyCode::ArrowDown, KeyCode::KeyS]),
            (Action::Left, vec![KeyCode::ArrowLeft, KeyCode::KeyA]),
            (Action::Right, vec![KeyCode::ArrowRight, KeyCode::KeyD]),
        ]
    });

pub(crate) fn all_key_codes() -> Vec<KeyCode> {
    let mut keys: Vec<KeyCode> = Vec::new();
    for (_action, key_codes) in &*ACTION_KEYMAP {
        for key in key_codes {
            keys.push(*key);
        }
    }
    keys
}

pub(crate) fn key_mappings_from_action(action: Action) -> Option<&'static Vec<KeyCode>> {
    for (action_key, key_codes) in &*ACTION_KEYMAP {
        if action == *action_key {
            return Some(key_codes);
        }
    }
    None
}
