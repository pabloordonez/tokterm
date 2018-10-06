use std::ptr::null_mut;
use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::Result;
use winapi::shared::windef::{HWND, POINT};
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{
    GetCursorPos, LoadCursorW, ScreenToClient, SetCursor, SetCursorPos, IDC_ARROW,
};
use Empty;

#[derive(Debug)]
pub struct WindowsMouse {
    window_handle: HWND,
}

impl WindowsMouse {
    pub fn new() -> WindowsMouse {
        WindowsMouse {
            window_handle: unsafe { GetConsoleWindow() },
        }
    }

    pub fn get_absolute_position(&self) -> Result<Point2d> {
        let mut point = POINT::empty();
        let success = unsafe { GetCursorPos(&mut point) };

        if success == 0 {
            return Err("Problems trying to obtain the cursor position.");
        }

        Ok(Point2d::new(point.x as i32, point.y as i32))
    }

    pub fn get_client_position(&self) -> Result<Point2d> {
        let position = self.get_absolute_position()?;
        let mut point = POINT {
            x: position.x as i32,
            y: position.y as i32,
        };

        let success = unsafe { ScreenToClient(self.window_handle, &mut point) };

        if success == 0 {
            return Err("Problems trying to obtain the client cursor position.");
        }

        Ok(Point2d::new(point.x as i32, point.y as i32))
    }

    pub fn set_position(&self, position: Point2d) -> Result<()> {
        let success = unsafe { SetCursorPos(position.x as i32, position.y as i32) };

        if success == 0 {
            return Err("Problems trying to set the cursor position.");
        }

        Ok(())
    }

    pub fn show_cursor(&self, visible: bool) -> Result<()> {
        if visible {
            unsafe { SetCursor(LoadCursorW(null_mut(), IDC_ARROW)) }
        } else {
            unsafe { SetCursor(null_mut()) }
        };

        Ok(())
    }
}
