use drawing::point_2d::Point2d;
use events::event::MouseEvent;

pub struct MouseState {
    pub left_button: bool,
    pub middle_button: bool,
    pub right_button: bool,
    pub extra_button_1: bool,
    pub extra_button_2: bool,
    pub extra_button_3: bool,
    pub extra_button_4: bool,
    pub position: Point2d,
}

impl MouseState {
    pub fn new() -> MouseState {
        MouseState {
            left_button: false,
            middle_button: false,
            right_button: false,
            extra_button_1: false,
            extra_button_2: false,
            extra_button_3: false,
            extra_button_4: false,
            position: Point2d::empty(),
        }
    }

    pub fn update_from_event(&mut self, mouse: MouseEvent) {
        self.left_button = mouse.left_button;
        self.middle_button = mouse.middle_button;
        self.right_button = mouse.right_button;
        self.extra_button_1 = mouse.extra_button_1;
        self.extra_button_2 = mouse.extra_button_2;
        self.extra_button_3 = mouse.extra_button_3;
        self.extra_button_4 = mouse.extra_button_4;
        self.position = mouse.position;
    }
}
