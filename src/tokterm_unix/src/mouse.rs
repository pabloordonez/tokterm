use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::system::mouse::Mouse;

pub struct UnixMouse {}

impl UnixMouse {
    pub fn new() -> UnixMouse {
        UnixMouse {}
    }
}

impl Mouse for UnixMouse {
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
