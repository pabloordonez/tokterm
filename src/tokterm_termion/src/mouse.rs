use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::system::mouse::Mouse;

pub struct TermionMouse {}

impl TermionMouse {
    pub fn new() -> TermionMouse {
        TermionMouse {}
    }
}

impl Mouse for TermionMouse {
    fn get_absolute_position(&self) -> Result<Point2d, &'static str> {
        unimplemented!()
    }

    fn get_client_position(&self) -> Result<Point2d, &'static str> {
        unimplemented!()
    }

    fn set_position(&self, position: Point2d) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn show_cursor(&self, visible: bool) -> Result<(), &'static str> {
        unimplemented!()
    }
}
