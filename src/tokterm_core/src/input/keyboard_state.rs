use events::event::{KeyboardEvent, KeyboardEventType};
use input::key::Key;

pub struct KeyboardState {
    pub keys: [bool; 175],
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState { keys: [false; 175] }
    }

    pub fn update_from_event(&mut self, keyboard: KeyboardEvent) {
        self.keys[keyboard.key.to_u32() as usize] =
            keyboard.event_type == KeyboardEventType::KeyDown;
        self.keys[Key::LeftShift.to_u32() as usize] = keyboard.left_shift;
        self.keys[Key::LeftControl.to_u32() as usize] = keyboard.left_control;
        self.keys[Key::LeftMenu.to_u32() as usize] = keyboard.left_menu;
        self.keys[Key::RightShift.to_u32() as usize] = keyboard.right_shift;
        self.keys[Key::RightControl.to_u32() as usize] = keyboard.right_control;
        self.keys[Key::RightMenu.to_u32() as usize] = keyboard.right_menu;
    }
}
