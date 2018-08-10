use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::drawing::size_2d::Size2d;
use tokterm_core::system::window::Window;

pub struct UnixWindow {}

impl UnixWindow {
    pub fn new() -> UnixWindow {
        UnixWindow {}
    }
}

impl Window for UnixWindow {
    fn get_window_client_size(&self) -> Result<Size2d, &'static str> {
        unimplemented!()
    }

    fn get_window_size(&self) -> Result<Size2d, &'static str> {
        unimplemented!()
    }

    fn set_window_size(&self, size: Size2d) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn get_window_position(&self) -> Result<Point2d, &'static str> {
        unimplemented!()
    }

    fn set_window_position(&self, position: Point2d) -> Result<(), &'static str> {
        unimplemented!()
    }
}
