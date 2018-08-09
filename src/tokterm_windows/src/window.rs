use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::drawing::size_2d::Size2d;
use tokterm_core::system::window::Window;
use tokterm_core::Result;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{GetClientRect, GetWindowRect, SetWindowPos};
use Empty;

#[derive(Debug)]
pub struct WindowsWindow {
    pub window_handle: HWND,
}

impl WindowsWindow {
    pub fn new() -> WindowsWindow {
        WindowsWindow {
            window_handle: unsafe { GetConsoleWindow() },
        }
    }
}

#[allow(dead_code)]
impl Window for WindowsWindow {
    fn get_window_size(&self) -> Result<Size2d> {
        let mut rect = RECT::empty();
        let success = unsafe { GetWindowRect(self.window_handle, &mut rect) };

        if success == 0 {
            return Err("Problems trying to obtain the window rect.");
        }

        Ok(Size2d::new(
            (rect.right - rect.left) as usize,
            (rect.bottom - rect.top) as usize,
        ))
    }

    fn get_window_client_size(&self) -> Result<Size2d> {
        let mut rect = RECT::empty();
        let success = unsafe { GetClientRect(self.window_handle, &mut rect) };

        if success == 0 {
            return Err("Problems trying to obtain the client rect.");
        }

        Ok(Size2d::new(
            (rect.right - rect.left) as usize,
            (rect.bottom - rect.top) as usize,
        ))
    }

    fn set_window_size(&self, size: Size2d) -> Result<()> {
        let mut rect = RECT::empty();
        let success = unsafe { GetWindowRect(self.window_handle, &mut rect) };

        if success == 0 {
            return Err("Problems trying to obtain the window rect.");
        }

        let success = unsafe {
            SetWindowPos(
                self.window_handle,
                0 as HWND,
                rect.top,
                rect.left,
                size.width as i32,
                size.height as i32,
                0x0020 | 0x0040,
            )
        };

        if success == 0 {
            return Err("Problem trying to set the windows size.");
        }

        Ok(())
    }

    fn get_window_position(&self) -> Result<Point2d> {
        let mut rect = RECT::empty();
        let success = unsafe { GetWindowRect(self.window_handle, &mut rect) };

        if success == 0 {
            return Err("Problems trying to obtain the window rect.");
        }

        Ok(Point2d::new(rect.left as usize, rect.top as usize))
    }

    fn set_window_position(&self, position: Point2d) -> Result<()> {
        let mut rect = RECT::empty();
        let success = unsafe { GetWindowRect(self.window_handle, &mut rect) };

        if success == 0 {
            return Err("Problems trying to obtain the window rect.");
        }

        let success = unsafe {
            SetWindowPos(
                self.window_handle,
                0 as HWND,
                position.x as i32,
                position.y as i32,
                rect.right - rect.left,
                rect.bottom - rect.top,
                0x0020 | 0x0040,
            )
        };

        if success == 0 {
            return Err("Problem trying to set the windows position.");
        }

        Ok(())
    }
}
