use drawing::point_2d::Point2d;
use drawing::size_2d::Size2d;
use input::key::Key;

/// Enumerates all the possible mouse event types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseEventType {
    MouseMove,
    Click,
    DoubleClick,
    Wheel,
}

/// Enumerates all the possible keyboard event types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardEventType {
    KeyDown,
    KeyUp,
}

/// Enumerates all the possible window event types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowEventType {
    WindowMove,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowClose,
}

/// Represents a mouse event like mouse move or mouse down.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct MouseEvent {
    pub event_type: MouseEventType,
    pub left_button: bool,
    pub middle_button: bool,
    pub right_button: bool,
    pub extra_button_1: bool,
    pub extra_button_2: bool,
    pub extra_button_3: bool,
    pub extra_button_4: bool,
    pub position: Point2d,
    pub wheel_delta: i16,
}

/// Represents a keyboard event like key down or key up.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct KeyboardEvent {
    pub event_type: KeyboardEventType,
    pub key: Key,
    pub key_code: u16,
    pub character: char,
    pub left_control: bool,
    pub left_shift: bool,
    pub left_menu: bool,
    pub right_control: bool,
    pub right_shift: bool,
    pub right_menu: bool,
}

/// Represents a window event like window moved or window resized.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct WindowEvent {
    pub event_type: WindowEventType,
    pub position: Point2d,
    pub size: Size2d,
}

/// Event object enumeration can be one of the valid event types.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Window(WindowEvent),
}
