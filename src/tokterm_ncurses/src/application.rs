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
