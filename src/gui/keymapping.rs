use std::collections::HashSet;
use egui::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Select,
    Start,
}

impl KeyAction {
    pub const ALL: [KeyAction; 8] = [
        KeyAction::Up,
        KeyAction::Down,
        KeyAction::Left,
        KeyAction::Right,
        KeyAction::A,
        KeyAction::B,
        KeyAction::Select,
        KeyAction::Start,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            KeyAction::Up => "Up",
            KeyAction::Down => "Down",
            KeyAction::Left => "Left",
            KeyAction::Right => "Right",
            KeyAction::A => "A",
            KeyAction::B => "B",
            KeyAction::Select => "Select",
            KeyAction::Start => "Start",
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct KeyInput {
    pub a_pushed: bool,
    pub b_pushed: bool,
    pub select_pushed: bool,
    pub start_pushed: bool,
    pub up_pushed: bool,
    pub down_pushed: bool,
    pub left_pushed: bool,
    pub right_pushed: bool,
}

impl From<&KeyInput> for bool {
    fn from(val: &KeyInput) -> Self {
        val.a_pushed
            || val.b_pushed
            || val.select_pushed
            || val.start_pushed
            || val.up_pushed
            || val.down_pushed
            || val.left_pushed
            || val.right_pushed
    }
}

#[derive(Clone)]
pub struct KeyMapping {
    pub a: Key,
    pub b: Key,
    pub select: Key,
    pub start: Key,
    pub up: Key,
    pub down: Key,
    pub left: Key,
    pub right: Key,
}

impl Default for KeyMapping {
    fn default() -> Self {
        KeyMapping {
            a: Key::J,
            b: Key::K,
            select: Key::N,
            start: Key::M,
            up: Key::W,
            down: Key::S,
            left: Key::A,
            right: Key::D,
        }
    }
}

impl KeyMapping {
    pub fn generate_key_input(&self, keys_down: HashSet<Key>) -> KeyInput {
        KeyInput {
            a_pushed: keys_down.contains(&self.a),
            b_pushed: keys_down.contains(&self.b),
            select_pushed: keys_down.contains(&self.select),
            start_pushed: keys_down.contains(&self.start),
            up_pushed: keys_down.contains(&self.up),
            down_pushed: keys_down.contains(&self.down),
            left_pushed: keys_down.contains(&self.left),
            right_pushed: keys_down.contains(&self.right),
        }
    }

    pub fn get(&self, slot: &str) -> Key {
        match slot {
            "Up" => self.up,
            "Down" => self.down,
            "Left" => self.left,
            "Right" => self.right,
            "A" => self.a,
            "B" => self.b,
            "Select" => self.select,
            "Start" => self.start,
            _ => unreachable!("unknown key slot: {slot}"),
        }
    }

    fn set_raw(&mut self, slot: &str, key: Key) {
        match slot {
            "Up" => self.up = key,
            "Down" => self.down = key,
            "Left" => self.left = key,
            "Right" => self.right = key,
            "A" => self.a = key,
            "B" => self.b = key,
            "Select" => self.select = key,
            "Start" => self.start = key,
            _ => unreachable!("unknown key slot: {slot}"),
        }
    }

    pub fn remap(&mut self, slot: &str, new_key: Key) {
        let old_key = self.get(slot);
        if old_key == new_key {
            return;
        }

        const ALL: [&str; 8] = ["Up", "Down", "Left", "Right", "A", "B", "Select", "Start"];

        if let Some(conflicting) = ALL.into_iter().find(|&s| s != slot && self.get(s) == new_key) {
            self.set_raw(conflicting, old_key);
        }
        self.set_raw(slot, new_key);
    }
}