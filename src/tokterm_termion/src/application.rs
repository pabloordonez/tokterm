use mouse::TermionMouse;
use terminal::TermionTerminal;
use termion::event::Key;
use termion::event::{Event, MouseButton, MouseEvent};
use termion::input::TermRead;
use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::events::event::{
    Event as TokEvent, KeyboardEvent as TokKeyboardEvent, KeyboardEventType,
    MouseEvent as TokMouseEvent, MouseEventType,
};
use tokterm_core::events::event_queue::EventQueue;
use tokterm_core::input::key::Key as TokKey;
use tokterm_core::input::keyboard_state::KeyboardState;
use tokterm_core::input::mouse_state::MouseState;
use tokterm_core::system::application::Application;
use tokterm_core::system::mouse::Mouse;
use tokterm_core::system::terminal::Terminal;
use tokterm_core::system::window::Window;
use tokterm_core::Result;
use window::TermionWindow;

pub struct TermionApplication {
    terminal: TermionTerminal,
    window: TermionWindow,
    mouse: TermionMouse,
    event_queue: EventQueue,
    mouse_state: MouseState,
    keyboard_state: KeyboardState,
}

impl TermionApplication {
    pub fn create() -> Result<TermionApplication> {
        let application = TermionApplication {
            terminal: TermionTerminal::create()?,
            window: TermionWindow::new(),
            mouse: TermionMouse::new(),
            event_queue: EventQueue::new(),
            mouse_state: MouseState::new(),
            keyboard_state: KeyboardState::new(),
        };

        Ok(application)
    }
}

impl Application for TermionApplication {
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
        let event = match self.terminal.get_stdin().events().next() {
            Some(event) => match event {
                Ok(event) => event,
                Err(_) => return Err("Couldn't retrieve the stdin events."),
            },
            None => return Ok(()),
        };

        let event = match event {
            Event::Key(key_event) => {
                let event = process_key_event(key_event)?;
                self.event_queue.add_event(event);
                event
            }
            Event::Mouse(mouse_event) => {
                let event = process_mouse_event(mouse_event)?;
                self.event_queue.add_event(event);
                event
            }
            _ => return Ok(()),
        };

        match event {
            TokEvent::Mouse(mouse) => self.mouse_state.update_from_event(mouse),
            TokEvent::Keyboard(keyboard) => self.keyboard_state.update_from_event(keyboard),
            TokEvent::Window(_) => (),
        }

        Ok(())
    }
}

fn process_mouse_event(mouse_event: MouseEvent) -> Result<TokEvent> {
    let (button, x, y) = match mouse_event {
        MouseEvent::Press(mouse_button, x, y) => (Some(mouse_button), x, y),
        MouseEvent::Release(x, y) => (None, x, y),
        MouseEvent::Hold(x, y) => (None, x, y),
    };

    Ok(TokEvent::Mouse(TokMouseEvent {
        event_type: MouseEventType::MouseMove,
        left_button: button == Some(MouseButton::Left),
        middle_button: button == Some(MouseButton::Middle),
        right_button: button == Some(MouseButton::Right),
        extra_button_1: false,
        extra_button_2: false,
        extra_button_3: false,
        extra_button_4: false,
        position: Point2d::new((x - 1) as usize, (y - 1) as usize),
        wheel_delta: match button {
            Some(MouseButton::WheelUp) => 1,
            Some(MouseButton::WheelDown) => -1,
            _ => 0,
        },
    }))
}

fn process_key_event(key_event: Key) -> Result<TokEvent> {
    Ok(TokEvent::Keyboard(TokKeyboardEvent {
        event_type: KeyboardEventType::KeyUp,
        key: get_key(key_event),
        key_code: 0,
        character: ' ',
        left_control: match key_event {
            Key::Ctrl(_) => true,
            _ => false,
        },
        left_shift: false,
        left_menu: match key_event {
            Key::Alt(_) => true,
            _ => false,
        },
        right_control: false,
        right_shift: false,
        right_menu: false,
    }))
}

fn get_key(key: Key) -> TokKey {
    match key {
        Key::Backspace => TokKey::Back,
        Key::Left => TokKey::Left,
        Key::Right => TokKey::Right,
        Key::Up => TokKey::Up,
        Key::Down => TokKey::Down,
        Key::Home => TokKey::Home,
        Key::End => TokKey::End,
        Key::PageUp => TokKey::Prior,
        Key::PageDown => TokKey::Next,
        Key::Delete => TokKey::Delete,
        Key::Insert => TokKey::Insert,
        Key::Esc => TokKey::Escape,

        Key::F(1) => TokKey::F1,
        Key::F(2) => TokKey::F2,
        Key::F(3) => TokKey::F3,
        Key::F(4) => TokKey::F4,
        Key::F(5) => TokKey::F5,
        Key::F(6) => TokKey::F6,
        Key::F(7) => TokKey::F7,
        Key::F(8) => TokKey::F8,
        Key::F(9) => TokKey::F9,
        Key::F(10) => TokKey::F10,
        Key::F(11) => TokKey::F11,
        Key::F(12) => TokKey::F12,

        Key::Char('a')
        | Key::Alt('a')
        | Key::Ctrl('a')
        | Key::Char('A')
        | Key::Alt('A')
        | Key::Ctrl('A') => TokKey::A,
        Key::Char('b')
        | Key::Alt('b')
        | Key::Ctrl('b')
        | Key::Char('B')
        | Key::Alt('B')
        | Key::Ctrl('B') => TokKey::B,
        Key::Char('c')
        | Key::Alt('c')
        | Key::Ctrl('c')
        | Key::Char('C')
        | Key::Alt('C')
        | Key::Ctrl('C') => TokKey::C,
        Key::Char('d')
        | Key::Alt('d')
        | Key::Ctrl('d')
        | Key::Char('D')
        | Key::Alt('D')
        | Key::Ctrl('D') => TokKey::D,
        Key::Char('e')
        | Key::Alt('e')
        | Key::Ctrl('e')
        | Key::Char('E')
        | Key::Alt('E')
        | Key::Ctrl('E') => TokKey::E,
        Key::Char('f')
        | Key::Alt('f')
        | Key::Ctrl('f')
        | Key::Char('F')
        | Key::Alt('F')
        | Key::Ctrl('F') => TokKey::F,
        Key::Char('g')
        | Key::Alt('g')
        | Key::Ctrl('g')
        | Key::Char('G')
        | Key::Alt('G')
        | Key::Ctrl('G') => TokKey::G,
        Key::Char('h')
        | Key::Alt('h')
        | Key::Ctrl('h')
        | Key::Char('H')
        | Key::Alt('H')
        | Key::Ctrl('H') => TokKey::H,
        Key::Char('i')
        | Key::Alt('i')
        | Key::Ctrl('i')
        | Key::Char('I')
        | Key::Alt('I')
        | Key::Ctrl('I') => TokKey::I,
        Key::Char('j')
        | Key::Alt('j')
        | Key::Ctrl('j')
        | Key::Char('J')
        | Key::Alt('J')
        | Key::Ctrl('J') => TokKey::J,
        Key::Char('k')
        | Key::Alt('k')
        | Key::Ctrl('k')
        | Key::Char('K')
        | Key::Alt('K')
        | Key::Ctrl('K') => TokKey::K,
        Key::Char('l')
        | Key::Alt('l')
        | Key::Ctrl('l')
        | Key::Char('L')
        | Key::Alt('L')
        | Key::Ctrl('L') => TokKey::L,
        Key::Char('m')
        | Key::Alt('m')
        | Key::Ctrl('m')
        | Key::Char('M')
        | Key::Alt('M')
        | Key::Ctrl('M') => TokKey::M,
        Key::Char('n')
        | Key::Alt('n')
        | Key::Ctrl('n')
        | Key::Char('N')
        | Key::Alt('N')
        | Key::Ctrl('N') => TokKey::N,
        Key::Char('o')
        | Key::Alt('o')
        | Key::Ctrl('o')
        | Key::Char('O')
        | Key::Alt('O')
        | Key::Ctrl('O') => TokKey::O,
        Key::Char('p')
        | Key::Alt('p')
        | Key::Ctrl('p')
        | Key::Char('P')
        | Key::Alt('P')
        | Key::Ctrl('P') => TokKey::P,
        Key::Char('q')
        | Key::Alt('q')
        | Key::Ctrl('q')
        | Key::Char('Q')
        | Key::Alt('Q')
        | Key::Ctrl('Q') => TokKey::Q,
        Key::Char('r')
        | Key::Alt('r')
        | Key::Ctrl('r')
        | Key::Char('R')
        | Key::Alt('R')
        | Key::Ctrl('R') => TokKey::R,
        Key::Char('s')
        | Key::Alt('s')
        | Key::Ctrl('s')
        | Key::Char('S')
        | Key::Alt('S')
        | Key::Ctrl('S') => TokKey::S,
        Key::Char('t')
        | Key::Alt('t')
        | Key::Ctrl('t')
        | Key::Char('T')
        | Key::Alt('T')
        | Key::Ctrl('T') => TokKey::T,
        Key::Char('u')
        | Key::Alt('u')
        | Key::Ctrl('u')
        | Key::Char('U')
        | Key::Alt('U')
        | Key::Ctrl('U') => TokKey::U,
        Key::Char('v')
        | Key::Alt('v')
        | Key::Ctrl('v')
        | Key::Char('V')
        | Key::Alt('V')
        | Key::Ctrl('V') => TokKey::V,
        Key::Char('w')
        | Key::Alt('w')
        | Key::Ctrl('w')
        | Key::Char('W')
        | Key::Alt('W')
        | Key::Ctrl('W') => TokKey::W,
        Key::Char('x')
        | Key::Alt('x')
        | Key::Ctrl('x')
        | Key::Char('X')
        | Key::Alt('X')
        | Key::Ctrl('X') => TokKey::X,
        Key::Char('y')
        | Key::Alt('y')
        | Key::Ctrl('y')
        | Key::Char('Y')
        | Key::Alt('Y')
        | Key::Ctrl('Y') => TokKey::Y,
        Key::Char('z')
        | Key::Alt('z')
        | Key::Ctrl('z')
        | Key::Char('Z')
        | Key::Alt('Z')
        | Key::Ctrl('Z') => TokKey::Z,

        Key::Char('0') | Key::Alt('0') | Key::Ctrl('0') => TokKey::Key0,
        Key::Char('1') | Key::Alt('1') | Key::Ctrl('1') => TokKey::Key1,
        Key::Char('2') | Key::Alt('2') | Key::Ctrl('2') => TokKey::Key2,
        Key::Char('3') | Key::Alt('3') | Key::Ctrl('3') => TokKey::Key3,
        Key::Char('4') | Key::Alt('4') | Key::Ctrl('4') => TokKey::Key4,
        Key::Char('5') | Key::Alt('5') | Key::Ctrl('5') => TokKey::Key5,
        Key::Char('6') | Key::Alt('6') | Key::Ctrl('6') => TokKey::Key6,
        Key::Char('7') | Key::Alt('7') | Key::Ctrl('7') => TokKey::Key7,
        Key::Char('8') | Key::Alt('8') | Key::Ctrl('8') => TokKey::Key8,
        Key::Char('9') | Key::Alt('9') | Key::Ctrl('9') => TokKey::Key9,

        Key::Alt(_) => TokKey::Menu,
        Key::Ctrl(_) => TokKey::Control,
        _ => TokKey::None,
    }
}
