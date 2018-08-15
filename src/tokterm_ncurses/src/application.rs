use mouse::NCursesMouse;
use ncurses::cbreak;
use ncurses::constants::BUTTON1_CLICKED;
use ncurses::constants::BUTTON1_DOUBLE_CLICKED;
use ncurses::constants::BUTTON1_PRESSED;
use ncurses::constants::BUTTON2_CLICKED;
use ncurses::constants::BUTTON2_DOUBLE_CLICKED;
use ncurses::constants::BUTTON2_PRESSED;
use ncurses::constants::BUTTON3_CLICKED;
use ncurses::constants::BUTTON3_DOUBLE_CLICKED;
use ncurses::constants::BUTTON3_PRESSED;
use ncurses::constants::BUTTON4_CLICKED;
use ncurses::constants::BUTTON4_DOUBLE_CLICKED;
use ncurses::constants::BUTTON4_PRESSED;
use ncurses::constants::ALL_MOUSE_EVENTS;
use ncurses::constants::ERR;
use ncurses::constants::KEY_MOUSE;
use ncurses::constants::REPORT_MOUSE_POSITION;
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
use tokterm_core::events::event::MouseEvent;
use tokterm_core::events::event::MouseEventType;
use tokterm_core::events::event_queue::EventQueue;
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

        print!("\033[?1003h\n");

        Ok(application)
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
                let event = process_mouse_event()?;
                self.event_queue.add_event(event);
                Some(event)
            }
            _ => None,
        };

        match event {
            Some(Event::Mouse(mouse)) => self.mouse_state.update_from_event(mouse),
            Some(Event::Keyboard(keyboard)) => self.keyboard_state.update_from_event(keyboard),
            _ => (),
        };

        Ok(())
    }
}

fn process_mouse_event() -> Result<Event> {
    let mut event: MEVENT = unsafe { zeroed() };

    if getmouse(&mut event) == ERR {
        return Err("Couldn't retrieve the mouse event.");
    }
    let mouse_event_type = if (event.bstate & BUTTON1_DOUBLE_CLICKED as u64) != 0
        || (event.bstate & BUTTON2_DOUBLE_CLICKED as u64) != 0
        || (event.bstate & BUTTON3_DOUBLE_CLICKED as u64) != 0
        || (event.bstate & BUTTON4_DOUBLE_CLICKED as u64) != 0
    {
        MouseEventType::DoubleClick
    } else if (event.bstate & BUTTON1_CLICKED as u64) != 0
        || (event.bstate & BUTTON2_CLICKED as u64) != 0
        || (event.bstate & BUTTON3_CLICKED as u64) != 0
        || (event.bstate & BUTTON4_CLICKED as u64) != 0
    {
        MouseEventType::DoubleClick
    } else {
        MouseEventType::MouseMove
    };

    Ok(Event::Mouse(MouseEvent {
        event_type: mouse_event_type,
        left_button: (event.bstate & BUTTON1_PRESSED as u64) != 0,
        middle_button: (event.bstate & BUTTON2_PRESSED as u64) != 0,
        right_button: (event.bstate & BUTTON3_PRESSED as u64) != 0,
        extra_button_1: (event.bstate & BUTTON4_PRESSED as u64) != 0,
        extra_button_2: false,
        extra_button_3: false,
        extra_button_4: false,
        position: Point2d::new(event.x as usize, event.y as usize),
        wheel_delta: 0,
    }))
}
