use events::event_queue::EventQueue;
use input::keyboard_state::KeyboardState;
use input::mouse_state::MouseState;
use system::mouse::Mouse;
use system::terminal::Terminal;
use system::window::Window;
use Result;

pub trait Application {
    fn get_terminal(&self) -> &Terminal;

    fn get_mut_terminal(&mut self) -> &mut Terminal;

    fn get_window(&self) -> &Window;

    fn get_mouse(&self) -> &Mouse;

    fn get_mouse_state(&self) -> &MouseState;

    fn get_keyboard_state(&self) -> &KeyboardState;

    fn get_event_queue(&self) -> &EventQueue;

    fn get_mut_event_queue(&mut self) -> &mut EventQueue;

    fn listen_events(&mut self) -> Result<()>;
}
