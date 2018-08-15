use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::events::event::{
    Event, KeyboardEvent, KeyboardEventType, MouseEvent, MouseEventType, WindowEvent,
    WindowEventType,
};
use tokterm_core::events::event_queue::EventQueue;
use tokterm_core::input::key::Key;
use tokterm_core::input::keyboard_state::KeyboardState;
use tokterm_core::input::mouse_state::MouseState;
use tokterm_core::system::application::Application;
use tokterm_core::system::mouse::Mouse;
use tokterm_core::system::terminal::Terminal;
use tokterm_core::system::window::Window;
use tokterm_core::Result;

use mouse::WindowsMouse;
use std::char::from_u32;
use terminal::WindowsTerminal;
use winapi::um::consoleapi::{
    GetConsoleMode, GetNumberOfConsoleInputEvents, ReadConsoleInputW, SetConsoleMode,
};
use winapi::um::wincon::LEFT_ALT_PRESSED;
use winapi::um::wincon::LEFT_CTRL_PRESSED;
use winapi::um::wincon::RIGHT_ALT_PRESSED;
use winapi::um::wincon::RIGHT_CTRL_PRESSED;
use winapi::um::wincon::{
    FROM_LEFT_1ST_BUTTON_PRESSED, FROM_LEFT_2ND_BUTTON_PRESSED, FROM_LEFT_3RD_BUTTON_PRESSED,
    FROM_LEFT_4TH_BUTTON_PRESSED, DOUBLE_CLICK, ENABLE_EXTENDED_FLAGS, ENABLE_MOUSE_INPUT,
    ENABLE_QUICK_EDIT_MODE, ENABLE_WINDOW_INPUT, FOCUS_EVENT, INPUT_RECORD, KEY_EVENT, MOUSE_EVENT,
    MOUSE_HWHEELED, MOUSE_MOVED, MOUSE_WHEELED, RIGHTMOST_BUTTON_PRESSED,
};

use winapi::um::winuser::GetKeyState;
use winapi::um::winuser::{VK_LSHIFT, VK_RSHIFT};
use window::WindowsWindow;
use Empty;

#[allow(dead_code)]
pub struct WindowsApplication {
    terminal: WindowsTerminal,
    window: WindowsWindow,
    mouse: WindowsMouse,
    event_queue: EventQueue,
    mouse_state: MouseState,
    keyboard_state: KeyboardState,
}

#[allow(dead_code)]
impl WindowsApplication {
    pub fn create() -> Result<WindowsApplication> {
        let application = WindowsApplication {
            window: WindowsWindow::new(),
            terminal: WindowsTerminal::create()?,
            mouse: WindowsMouse::new(),
            event_queue: EventQueue::new(),
            mouse_state: MouseState::new(),
            keyboard_state: KeyboardState::new(),
        };

        let mut console_mode = 0;
        let success =
            unsafe { GetConsoleMode(application.terminal.input_handle, &mut console_mode) };

        if success == -1 {
            return Err("Couldn't retrieve the console mode.");
        }

        console_mode &= !ENABLE_QUICK_EDIT_MODE;
        console_mode |= ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT | ENABLE_EXTENDED_FLAGS;

        let success = unsafe { SetConsoleMode(application.terminal.input_handle, console_mode) };

        if success == -1 {
            return Err("Couldn't set the console mode.");
        }

        Ok(application)
    }
}

impl Application for WindowsApplication {
    #[inline]
    fn get_terminal(&self) -> &Terminal {
        &self.terminal
    }

    #[inline]
    fn get_mut_terminal(&mut self) -> &mut Terminal {
        &mut self.terminal
    }

    #[inline]
    fn get_window(&self) -> &Window {
        &self.window
    }

    #[inline]
    fn get_mouse(&self) -> &Mouse {
        &self.mouse
    }

    #[inline]
    fn get_mouse_state(&self) -> &MouseState {
        &self.mouse_state
    }

    #[inline]
    fn get_keyboard_state(&self) -> &KeyboardState {
        &self.keyboard_state
    }

    #[inline]
    fn get_event_queue(&self) -> &EventQueue {
        &self.event_queue
    }

    #[inline]
    fn get_mut_event_queue(&mut self) -> &mut EventQueue {
        &mut self.event_queue
    }

    fn listen_events(&mut self) -> Result<()> {
        let mut input_records = [INPUT_RECORD::empty(); 128];
        let mut events_read: u32 = 0;

        let success =
            unsafe { GetNumberOfConsoleInputEvents(self.terminal.input_handle, &mut events_read) };

        if success == -1 {
            return Err("Couldn't determine the amount of unread events");
        }

        if events_read <= 0 {
            return Ok(());
        }

        let success = unsafe {
            ReadConsoleInputW(
                self.terminal.input_handle,
                input_records.as_mut_ptr(),
                128,
                &mut events_read,
            )
        };

        if success == -1 {
            return Err("Couldn't retrieve the console window events.");
        }

        for input_record in input_records.iter() {
            let event = match input_record.EventType {
                KEY_EVENT => {
                    let event = process_key_event(input_record);
                    self.event_queue.add_event(event);
                    event
                }
                MOUSE_EVENT => {
                    let event = process_mouse_event(input_record);
                    self.event_queue.add_event(event);
                    event
                }
                FOCUS_EVENT => {
                    let event = process_window_event(&self.window)?;
                    self.event_queue.add_event(event);
                    event
                }
                _ => continue,
            };

            match event {
                Event::Mouse(mouse) => self.mouse_state.update_from_event(mouse),
                Event::Keyboard(keyboard) => self.keyboard_state.update_from_event(keyboard),
                Event::Window(_) => (),
            }
        }

        Ok(())
    }
}

#[inline]
fn process_key_event(input_record: &INPUT_RECORD) -> Event {
    let keyboard_event = unsafe { input_record.Event.KeyEvent() };

    Event::Keyboard(KeyboardEvent {
        event_type: if keyboard_event.bKeyDown == -1 {
            KeyboardEventType::KeyUp
        } else {
            KeyboardEventType::KeyDown
        },
        key: get_key(keyboard_event.wVirtualKeyCode),
        key_code: keyboard_event.wVirtualKeyCode,
        character: get_char_from_u16(unsafe { *keyboard_event.uChar.UnicodeChar() }),
        left_control: keyboard_event.dwControlKeyState & LEFT_CTRL_PRESSED != 0,
        left_shift: unsafe { GetKeyState(VK_LSHIFT) as u16 } & 0x8000 != 0,
        left_menu: keyboard_event.dwControlKeyState & LEFT_ALT_PRESSED != 0,
        right_control: keyboard_event.dwControlKeyState & RIGHT_CTRL_PRESSED != 0,
        right_shift: unsafe { GetKeyState(VK_RSHIFT) as u16 } & 0x8000 != 0,
        right_menu: keyboard_event.dwControlKeyState & RIGHT_ALT_PRESSED != 0,
    })
}

#[inline]
fn process_mouse_event(input_record: &INPUT_RECORD) -> Event {
    let mouse_event = unsafe { input_record.Event.MouseEvent() };

    Event::Mouse(MouseEvent {
        event_type: match mouse_event.dwEventFlags {
            0 => MouseEventType::Click,
            MOUSE_MOVED => MouseEventType::MouseMove,
            MOUSE_WHEELED => MouseEventType::HorizontalWheel,
            MOUSE_HWHEELED => MouseEventType::VerticalWheel,
            DOUBLE_CLICK => MouseEventType::DoubleClick,
            _ => MouseEventType::MouseMove,
        },
        left_button: mouse_event.dwButtonState & FROM_LEFT_1ST_BUTTON_PRESSED != 0,
        middle_button: mouse_event.dwButtonState & FROM_LEFT_2ND_BUTTON_PRESSED != 0,
        right_button: mouse_event.dwButtonState & RIGHTMOST_BUTTON_PRESSED != 0,
        extra_button_1: mouse_event.dwButtonState & FROM_LEFT_3RD_BUTTON_PRESSED != 0,
        extra_button_2: mouse_event.dwButtonState & FROM_LEFT_4TH_BUTTON_PRESSED != 0,
        extra_button_3: false,
        extra_button_4: false,
        wheel_delta: get_wheel_delta(mouse_event.dwButtonState),
        position: Point2d::new(
            mouse_event.dwMousePosition.X as usize,
            mouse_event.dwMousePosition.Y as usize,
        ),
    })
}

#[inline]
fn process_window_event(window: &WindowsWindow) -> Result<Event> {
    Ok(Event::Window(WindowEvent {
        event_type: WindowEventType::WindowFocus,
        position: window.get_window_position()?,
        size: window.get_window_size()?,
    }))
}

#[inline]
fn get_wheel_delta(button_state: u32) -> i16 {
    (button_state >> 16) as i16
}

#[inline]
fn get_char_from_u16(unicode: u16) -> char {
    match from_u32(unicode as u32) {
        Some(character) => character,
        None => ' ',
    }
}

fn get_key(virtual_key_code: u16) -> Key {
    match virtual_key_code {
        // VK_LBUTTON
        0x01 => Key::LeftButton,

        // VK_RBUTTON
        0x02 => Key::RightButton,

        // VK_CANCEL
        0x03 => Key::Cancel,

        // VK_MBUTTON
        0x04 => Key::MiddleButton,

        // VK_XBUTTON1
        0x05 => Key::XButton1,

        // VK_XBUTTON2
        0x06 => Key::XButton2,

        // VK_BACK
        0x08 => Key::Back,

        // VK_TAB
        0x09 => Key::Tab,

        // VK_CLEAR
        0x0C => Key::Clear,

        // VK_RETURN
        0x0D => Key::Return,

        // VK_SHIFT
        0x10 => Key::Shift,

        // VK_CONTROL
        0x11 => Key::Control,

        // VK_MENU
        0x12 => Key::Menu,

        // VK_PAUSE
        0x13 => Key::Pause,

        // VK_CAPITAL
        0x14 => Key::Capital,

        // VK_KANA
        0x15 => Key::KanaHangelHangul,

        // VK_JUNJA
        0x17 => Key::Junja,

        // VK_FINAL
        0x18 => Key::Final,

        // VK_HANJA
        0x19 => Key::HanjaKanji,

        // VK_ESCAPE
        0x1B => Key::Escape,

        // VK_CONVERT
        0x1C => Key::Convert,

        // VK_NONCONVERT
        0x1D => Key::NonConvert,

        // VK_ACCEPT
        0x1E => Key::Accept,

        // VK_MODECHANGE
        0x1F => Key::ModeChange,

        // VK_SPACE
        0x20 => Key::Space,

        // VK_PRIOR
        0x21 => Key::Prior,

        // VK_NEXT
        0x22 => Key::Next,

        // VK_END
        0x23 => Key::End,

        // VK_HOME
        0x24 => Key::Home,

        // VK_LEFT
        0x25 => Key::Left,

        // VK_UP
        0x26 => Key::Up,

        // VK_RIGHT
        0x27 => Key::Right,

        // VK_DOWN
        0x28 => Key::Down,

        // VK_SELECT
        0x29 => Key::Select,

        // VK_PRINT
        0x2A => Key::Print,

        // VK_EXECUTE
        0x2B => Key::Execute,

        // VK_SNAPSHOT
        0x2C => Key::Snapshot,

        // VK_INSERT
        0x2D => Key::Insert,

        // VK_DELETE
        0x2E => Key::Delete,

        // VK_HELP
        0x2F => Key::Help,

        // 0 key
        0x30 => Key::Key0,

        // 1 key
        0x31 => Key::Key1,

        // 2 key
        0x32 => Key::Key2,

        // 3 key
        0x33 => Key::Key3,

        // 4 key
        0x34 => Key::Key4,

        // 5 key
        0x35 => Key::Key5,

        // 6 key
        0x36 => Key::Key6,

        // 7 key
        0x37 => Key::Key7,

        // 8 key
        0x38 => Key::Key8,

        // 9 key
        0x39 => Key::Key9,

        // A key
        0x41 => Key::A,

        // B key
        0x42 => Key::B,

        // C key
        0x43 => Key::C,

        // D key
        0x44 => Key::D,

        // E key
        0x45 => Key::E,

        // F key
        0x46 => Key::F,

        // G key
        0x47 => Key::G,

        // H key
        0x48 => Key::H,

        // I key
        0x49 => Key::I,

        // J key
        0x4A => Key::J,

        // K key
        0x4B => Key::K,

        // L key
        0x4C => Key::L,

        // M key
        0x4D => Key::M,

        // N key
        0x4E => Key::N,

        // O key
        0x4F => Key::O,

        // P key
        0x50 => Key::P,

        // Q key
        0x51 => Key::Q,

        // R key
        0x52 => Key::R,

        // S key
        0x53 => Key::S,

        // T key
        0x54 => Key::T,

        // U key
        0x55 => Key::U,

        // V key
        0x56 => Key::V,

        // W key
        0x57 => Key::W,

        // X key
        0x58 => Key::X,

        // Y key
        0x59 => Key::Y,

        // Z key
        0x5A => Key::Z,

        // VK_LWIN
        0x5B => Key::LeftWin,

        // VK_RWIN
        0x5C => Key::RightWin,

        // VK_APPS
        0x5D => Key::Apps,

        // VK_SLEEP
        0x5F => Key::Sleep,

        // VK_NUMPAD0
        0x60 => Key::NumPad0,

        // VK_NUMPAD1
        0x61 => Key::NumPad1,

        // VK_NUMPAD2
        0x62 => Key::NumPad2,

        // VK_NUMPAD3
        0x63 => Key::NumPad3,

        // VK_NUMPAD4
        0x64 => Key::NumPad4,

        // VK_NUMPAD5
        0x65 => Key::NumPad5,

        // VK_NUMPAD6
        0x66 => Key::NumPad6,

        // VK_NUMPAD7
        0x67 => Key::NumPad7,

        // VK_NUMPAD8
        0x68 => Key::NumPad8,

        // VK_NUMPAD9
        0x69 => Key::NumPad9,

        // VK_MULTIPLY
        0x6A => Key::Multiply,

        // VK_ADD
        0x6B => Key::Add,

        // VK_SEPARATOR
        0x6C => Key::Separator,

        // VK_SUBTRACT
        0x6D => Key::Subtract,

        // VK_DECIMAL
        0x6E => Key::Decimal,

        // VK_DIVIDE
        0x6F => Key::Divide,

        // VK_F1
        0x70 => Key::F1,

        // VK_F2
        0x71 => Key::F2,

        // VK_F3
        0x72 => Key::F3,

        // VK_F4
        0x73 => Key::F4,

        // VK_F5
        0x74 => Key::F5,

        // VK_F6
        0x75 => Key::F6,

        // VK_F7
        0x76 => Key::F7,

        // VK_F8
        0x77 => Key::F8,

        // VK_F9
        0x78 => Key::F9,

        // VK_F10
        0x79 => Key::F10,

        // VK_F11
        0x7A => Key::F11,

        // VK_F12
        0x7B => Key::F12,

        // VK_F13
        0x7C => Key::F13,

        // VK_F14
        0x7D => Key::F14,

        // VK_F15
        0x7E => Key::F15,

        // VK_F16
        0x7F => Key::F16,

        // VK_F17
        0x80 => Key::F17,

        // VK_F18
        0x81 => Key::F18,

        // VK_F19
        0x82 => Key::F19,

        // VK_F20
        0x83 => Key::F20,

        // VK_F21
        0x84 => Key::F21,

        // VK_F22
        0x85 => Key::F22,

        // VK_F23
        0x86 => Key::F23,

        // VK_F24
        0x87 => Key::F24,

        // VK_NUMLOCK
        0x90 => Key::NumLock,

        // VK_SCROLL
        0x91 => Key::Scroll,

        // VK_LSHIFT
        0xA0 => Key::LeftShift,

        // VK_RSHIFT
        0xA1 => Key::RightShift,

        // VK_LCONTROL
        0xA2 => Key::LeftControl,

        // VK_RCONTROL
        0xA3 => Key::RightControl,

        // VK_LMENU
        0xA4 => Key::LeftMenu,

        // VK_RMENU
        0xA5 => Key::RightMenu,

        // VK_BROWSER_BACK
        0xA6 => Key::BrowserBack,

        // VK_BROWSER_FORWARD
        0xA7 => Key::BrowserForward,

        // VK_BROWSER_REFRESH
        0xA8 => Key::BrowserRefresh,

        // VK_BROWSER_STOP
        0xA9 => Key::BrowserStop,

        // VK_BROWSER_SEARCH
        0xAA => Key::BrowserSearch,

        // VK_BROWSER_FAVORITES
        0xAB => Key::BrowserFavorites,

        // VK_BROWSER_HOME
        0xAC => Key::BrowserHome,

        // VK_VOLUME_MUTE
        0xAD => Key::VolumeMute,

        // VK_VOLUME_DOWN
        0xAE => Key::VolumeDown,

        // VK_VOLUME_UP
        0xAF => Key::VolumeUp,

        // VK_MEDIA_NEXT_TRACK
        0xB0 => Key::MediaNextTrack,

        // VK_MEDIA_PREV_TRACK
        0xB1 => Key::MediaPreviousTrack,

        // VK_MEDIA_STOP
        0xB2 => Key::MediaStop,

        // VK_MEDIA_PLAY_PAUSE
        0xB3 => Key::MediaPlayPause,

        // VK_LAUNCH_MAIL
        0xB4 => Key::LaunchMail,

        // VK_LAUNCH_MEDIA_SELECT
        0xB5 => Key::LaunchMediaSelect,

        // VK_LAUNCH_APP1
        0xB6 => Key::LaunchApp1,

        // VK_LAUNCH_APP2
        0xB7 => Key::LaunchApp2,

        // VK_OEM_1
        0xBA => Key::Oem1,

        // VK_OEM_PLUS
        0xBB => Key::Plus,

        // VK_OEM_COMMA
        0xBC => Key::Comma,

        // VK_OEM_MINUS
        0xBD => Key::Minus,

        // VK_OEM_PERIOD
        0xBE => Key::Period,

        // VK_OEM_2
        0xBF => Key::Oem2,

        // VK_OEM_3
        0xC0 => Key::Oem3,

        // VK_OEM_4
        0xDB => Key::Oem4,

        // VK_OEM_5
        0xDC => Key::Oem5,

        // VK_OEM_6
        0xDD => Key::Oem6,

        // VK_OEM_7
        0xDE => Key::Oem7,

        // VK_OEM_8
        0xDF => Key::Oem8,

        // VK_PROCESSKEY
        0xE5 => Key::ProcessKey,

        // IME PROCESS key
        0xE6 => Key::ImeProcess,

        // VK_PACKET
        0xE7 => Key::Packet,

        // VK_ATTN
        0xF6 => Key::Attn,

        // VK_CRSEL
        0xF7 => Key::CrSel,

        // VK_EXSEL
        0xF8 => Key::ExSel,

        // VK_EREOF
        0xF9 => Key::EraseEof,

        // VK_PLAY
        0xFA => Key::Play,

        // VK_ZOOM
        0xFB => Key::Zoom,

        // VK_PA1
        0xFD => Key::PA1,

        // VK_OEM_CLEAR
        0xFE => Key::Clear,

        // Not Recognized
        _ => Key::None,
    }
}
