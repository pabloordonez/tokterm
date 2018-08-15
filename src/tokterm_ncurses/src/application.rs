use mouse::NCursesMouse;
use ncurses::cbreak;
use ncurses::constants::BUTTON1_CLICKED;
use ncurses::constants::BUTTON1_DOUBLE_CLICKED;
use ncurses::constants::BUTTON1_PRESSED;
use ncurses::constants::BUTTON1_RELEASED;
use ncurses::constants::BUTTON2_CLICKED;
use ncurses::constants::BUTTON2_DOUBLE_CLICKED;
use ncurses::constants::BUTTON2_PRESSED;
use ncurses::constants::BUTTON2_RELEASED;
use ncurses::constants::BUTTON3_CLICKED;
use ncurses::constants::BUTTON3_DOUBLE_CLICKED;
use ncurses::constants::BUTTON3_PRESSED;
use ncurses::constants::BUTTON3_RELEASED;
use ncurses::constants::BUTTON4_CLICKED;
use ncurses::constants::BUTTON4_DOUBLE_CLICKED;
use ncurses::constants::BUTTON4_PRESSED;
use ncurses::constants::BUTTON4_RELEASED;
use ncurses::constants::BUTTON5_CLICKED;
use ncurses::constants::BUTTON5_DOUBLE_CLICKED;
use ncurses::constants::BUTTON5_PRESSED;
use ncurses::constants::BUTTON5_RELEASED;
use ncurses::constants::ALL_MOUSE_EVENTS;
use ncurses::constants::ERR;
use ncurses::constants::KEY_MOUSE;
use ncurses::constants::REPORT_MOUSE_POSITION;
use ncurses::constants::{
    KEY_F0, KEY_F1, KEY_F10, KEY_F11, KEY_F12, KEY_F13, KEY_F14, KEY_F15, KEY_F2, KEY_F3, KEY_F4,
    KEY_F5, KEY_F6, KEY_F7, KEY_F8, KEY_F9, KEY_BACKSPACE, KEY_BREAK, KEY_CANCEL, KEY_CLEAR,
    KEY_DC, KEY_DOWN, KEY_EIC, KEY_ENTER, KEY_HOME, KEY_NPAGE, KEY_PPAGE, KEY_PRINT, KEY_RIGHT,
    KEY_SLEFT, KEY_UP,
};
use ncurses::getmouse;
use ncurses::has_colors;
use ncurses::keypad;
use ncurses::mousemask;
use ncurses::nodelay;
use ncurses::noecho;
use ncurses::start_color;
use ncurses::wgetch;
use ncurses::MEVENT;
use std::mem::zeroed;
use terminal::NCursesTerminal;
use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::events::event::Event;
use tokterm_core::events::event::KeyboardEvent;
use tokterm_core::events::event::KeyboardEventType;
use tokterm_core::events::event::MouseEvent;
use tokterm_core::events::event::MouseEventType;
use tokterm_core::events::event_queue::EventQueue;
use tokterm_core::input::key::Key;
use tokterm_core::input::keyboard_state::KeyboardState;
use tokterm_core::input::mouse_state::MouseState;
use tokterm_core::system::application::Application;
use tokterm_core::system::mouse::Mouse;
use tokterm_core::system::terminal::Terminal;
use tokterm_core::system::window::Window;
use tokterm_core::Result;
use window::NCursesWindow;

pub struct NCursesApplication {
    terminal: NCursesTerminal,
    window: NCursesWindow,
    mouse: NCursesMouse,
    event_queue: EventQueue,
    mouse_state: MouseState,
    keyboard_state: KeyboardState,

    /////////////////////////////////////////
    // mouse state hack
    /////////////////////////////////////////
    left_button: bool,
    middle_button: bool,
    right_button: bool,
    extra_button_1: bool,
    position: Point2d,
}

impl NCursesApplication {
    pub fn create() -> Result<NCursesApplication> {
        let application = NCursesApplication {
            terminal: NCursesTerminal::create()?,
            window: NCursesWindow::new(),
            mouse: NCursesMouse::new(),
            event_queue: EventQueue::new(),
            mouse_state: MouseState::new(),
            keyboard_state: KeyboardState::new(),
            left_button: false,
            middle_button: false,
            right_button: false,
            extra_button_1: false,
            position: Point2d::empty(),
        };

        if cbreak() == ERR {
            return Err("Couldn't change the input mode.");
        }

        if noecho() == ERR {
            return Err("Couldn't deactivate echo.");
        }

        if nodelay(application.terminal.get_window(), true) == ERR {
            return Err("Couldn't activate the no-delay option.");
        }

        if keypad(application.terminal.get_window(), true) == ERR {
            return Err("Couldn't enable keypad.");
        }

        if !has_colors() {
            return Err("The terminal does not support color.");
        }

        start_color();

        mousemask(
            (ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION) as u64,
            Option::None,
        );

        print!("\x1b[?1003h\n");

        Ok(application)
    }

    fn process_key_event(&self, key_event: i32) -> Option<Event> {
        let key = get_key(key_event);

        if key == Key::None {
            return Option::None;
        }

        Some(Event::Keyboard(KeyboardEvent {
            event_type: KeyboardEventType::KeyDown,
            key,
            key_code: key_event as u16,
            character: get_character(key_event),
            left_control: false,
            left_shift: false,
            left_menu: false,
            right_control: false,
            right_shift: false,
            right_menu: false,
        }))
    }

    fn process_mouse_event(&mut self) -> Result<Event> {
        let mut event: MEVENT = unsafe { zeroed() };

        if getmouse(&mut event) == ERR {
            return Err("Couldn't retrieve the mouse event.");
        }

        let release_1 = (event.bstate & 0b0000_0000_0000_0000_0001) != 0;
        let press_1 = (event.bstate & 0b0000_0000_0000_0000_0010) != 0;
        let click_1 = (event.bstate & 0b0000_0000_0000_0000_0100) != 0;
        let double_click_1 = (event.bstate & 0b0000_0000_0000_0000_1000) != 0;
        let triple_click_1 = (event.bstate & 0b0000_0000_0000_0001_0000) != 0;

        let release_2 = (event.bstate & 0b0000_0000_0000_0100_0000) != 0;
        let press_2 = (event.bstate & 0b0000_0000_0000_1000_0000) != 0;
        let click_2 = (event.bstate & 0b0000_0000_0001_0000_0000) != 0;

        let release_3 = (event.bstate & 0b0000_0000_0010_0000_0000) != 0;
        let press_3 = (event.bstate & 0b0000_0000_0100_0000_0000) != 0;
        let click_3 = (event.bstate & 0b0000_0000_1000_0000_0000) != 0;

        let release_4 = (event.bstate & 0b0000_0001_0000_0000_0000) != 0;
        let press_4 = (event.bstate & 0b0000_0010_0000_0000_0000) != 0;
        let click_4 = (event.bstate & 0b0000_0100_0000_0000_0000) != 0;
        let double_click_4 = (event.bstate & 0b0000_1000_0000_0000_0000) != 0;
        let triple_click_4 = (event.bstate & 0b0001_0000_0000_0000_0000) != 0;

        // wheel up 0x80000
        let wheel_up = (event.bstate & 0x80000) != 0;
        let wheel_down = (event.bstate & 0x8000000) != 0
            && event.x as usize == self.position.x
            && event.y as usize == self.position.y;

        let mouse_event_type = if click_1 || click_2 || click_3 || click_4 {
            MouseEventType::Click
        } else if double_click_1 || double_click_4 || triple_click_1 || triple_click_4 {
            MouseEventType::DoubleClick
        } else if wheel_down || wheel_up {
            MouseEventType::Wheel
        } else {
            MouseEventType::MouseMove
        };

        if press_1 {
            self.left_button = true;
        }

        if release_1 {
            self.left_button = false;
        }

        if press_2 {
            self.middle_button = true;
        }

        if release_2 {
            self.middle_button = false;
        }

        if press_3 {
            self.extra_button_1 = true;
        }

        if release_3 {
            self.extra_button_1 = false;
        }

        if press_4 {
            self.right_button = true;
        }

        if release_4 {
            self.right_button = false;
        }

        self.position.x = event.x as usize;
        self.position.y = event.y as usize;

        Ok(Event::Mouse(MouseEvent {
            event_type: mouse_event_type,
            left_button: self.left_button || click_1 || double_click_1 || triple_click_1,
            middle_button: self.middle_button || click_2,
            extra_button_1: self.extra_button_1 || click_3,
            right_button: self.right_button || click_4 || double_click_4 || triple_click_4,
            extra_button_2: false,
            extra_button_3: false,
            extra_button_4: false,
            position: Point2d::new(event.x as usize, event.y as usize),
            wheel_delta: if wheel_up {
                -1
            } else if wheel_down {
                1
            } else {
                0
            },
        }))
    }
}

impl Application for NCursesApplication {
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
        let c = wgetch(self.terminal.get_window());
        let event = match c {
            KEY_MOUSE => {
                let event = self.process_mouse_event()?;
                self.event_queue.add_event(event);
                Some(event)
            }
            _ => {
                let event = match self.process_key_event(c) {
                    Some(event) => event,
                    None => return Ok(()),
                };
                self.event_queue.add_event(event);
                Some(event)
            }
        };

        match event {
            Some(Event::Mouse(mouse)) => self.mouse_state.update_from_event(mouse),
            Some(Event::Keyboard(keyboard)) => self.keyboard_state.update_from_event(keyboard),
            _ => (),
        };

        Ok(())
    }
}

fn get_key(event: i32) -> Key {
    match event {
        KEY_BREAK => Key::Pause,    /* Break key(unreliable) */
        KEY_DOWN => Key::Down,      /* down-arrow key */
        KEY_UP => Key::Up,          /* up-arrow key */
        KEY_LEFT => Key::Left,      /* left-arrow key */
        KEY_RIGHT => Key::Right,    /* right-arrow key */
        KEY_HOME => Key::Home,      /* home key */
        KEY_BACKSPACE => Key::Back, /* backspace key */
        KEY_F1 => Key::F1,
        KEY_F2 => Key::F2,
        KEY_F3 => Key::F3,
        KEY_F4 => Key::F4,
        KEY_F5 => Key::F5,
        KEY_F6 => Key::F6,
        KEY_F7 => Key::F7,
        KEY_F8 => Key::F8,
        KEY_F9 => Key::F9,
        KEY_F10 => Key::F10,
        KEY_F11 => Key::F11,
        KEY_F12 => Key::F12,
        KEY_F13 => Key::F13,
        KEY_F14 => Key::F14,
        KEY_F15 => Key::F15,
        KEY_DC => Key::Delete,     /* delete-character key */
        KEY_EIC => Key::Insert,    /* insert-character key */
        KEY_CLEAR => Key::Clear,   /* clear-screen or erase key */
        KEY_NPAGE => Key::Next,    /* next-page key */
        KEY_PPAGE => Key::Prior,   /* previous-page key */
        KEY_ENTER => Key::Return,  /* enter/send key */
        KEY_PRINT => Key::Print,   /* print key */
        KEY_CANCEL => Key::Cancel, /* cancel key */
        65 | 97 => Key::A,
        66 | 98 => Key::B,
        67 | 99 => Key::C,
        68 | 100 => Key::D,
        69 | 101 => Key::E,
        70 | 102 => Key::F,
        71 | 103 => Key::G,
        72 | 104 => Key::H,
        73 | 105 => Key::I,
        74 | 106 => Key::J,
        75 | 107 => Key::K,
        76 | 108 => Key::L,
        77 | 109 => Key::M,
        78 | 110 => Key::N,
        79 | 111 => Key::O,
        80 | 112 => Key::P,
        81 | 113 => Key::Q,
        82 | 114 => Key::R,
        83 | 115 => Key::S,
        84 | 116 => Key::T,
        85 | 117 => Key::U,
        86 | 118 => Key::V,
        87 | 119 => Key::W,
        88 | 120 => Key::X,
        89 | 121 => Key::Y,
        90 | 122 => Key::Z,
        48 => Key::Key0,
        49 => Key::Key1,
        50 => Key::Key2,
        51 => Key::Key3,
        52 => Key::Key4,
        53 => Key::Key5,
        54 => Key::Key6,
        55 => Key::Key7,
        56 => Key::Key8,
        57 => Key::Key9,
        9 => Key::Tab,
        _ => Key::None,
    }
}

fn get_character(event: i32) -> char {
    match event {
        65 => 'A',
        66 => 'B',
        67 => 'C',
        68 => 'D',
        69 => 'E',
        70 => 'F',
        71 => 'G',
        72 => 'H',
        73 => 'I',
        74 => 'J',
        75 => 'K',
        76 => 'L',
        77 => 'M',
        78 => 'N',
        79 => 'O',
        80 => 'P',
        81 => 'Q',
        82 => 'R',
        83 => 'S',
        84 => 'T',
        85 => 'U',
        86 => 'V',
        87 => 'W',
        88 => 'X',
        89 => 'Y',
        90 => 'Z',
        97 => 'a',
        98 => 'b',
        99 => 'c',
        100 => 'd',
        101 => 'e',
        102 => 'f',
        103 => 'g',
        104 => 'h',
        105 => 'i',
        106 => 'j',
        107 => 'k',
        108 => 'l',
        109 => 'm',
        110 => 'n',
        111 => 'o',
        112 => 'p',
        113 => 'q',
        114 => 'r',
        115 => 's',
        116 => 't',
        117 => 'u',
        118 => 'v',
        119 => 'w',
        120 => 'x',
        121 => 'y',
        122 => 'z',
        48 => '0',
        49 => '1',
        50 => '2',
        51 => '3',
        52 => '4',
        53 => '5',
        54 => '6',
        55 => '7',
        56 => '8',
        57 => '9',
        _ => ' ',
    }
}
