use std::ptr::null_mut;
use tokterm_core::drawing::cell::Cell;
use tokterm_core::drawing::cell_buffer::CellBuffer;
use tokterm_core::drawing::point_2d::Point2d;
use tokterm_core::drawing::size_2d::Size2d;
use tokterm_core::system::terminal::Terminal;
use tokterm_core::system::window::Window;
use tokterm_core::Result;
use color::get_u16_from_color;
use winapi::ctypes::c_void;
use winapi::shared::windef::HWND;
use winapi::um::fileapi::CreateFileW;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_INPUT_HANDLE;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::wincon::{
    GetConsoleCursorInfo, GetConsoleScreenBufferInfo, GetConsoleWindow, SetConsoleCursorInfo,
    SetConsoleCursorPosition, WriteConsoleOutputW, CHAR_INFO, CONSOLE_CURSOR_INFO,
    CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT,
};
use winapi::um::winnt::HANDLE;
use {get_wstring, Empty};

#[derive(Debug)]
pub struct WindowsTerminal {
    pub console_handle: *mut c_void,
    pub output_handle: HANDLE,
    pub input_handle: HANDLE,
    pub window_handle: HWND,
}

impl WindowsTerminal {
    pub fn create() -> Result<WindowsTerminal> {
        let console_handle = unsafe {
            CreateFileW(
                get_wstring("CONOUT$").as_ptr(),
                0x40000000,
                2,
                null_mut(),
                3,
                0,
                null_mut(),
            )
        };

        if console_handle == null_mut() {
            return Err("Couldn't open the console output file.");
        }

        let output_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

        if output_handle == null_mut() {
            return Err("Couldn't retrieve the output handle.");
        }

        let input_handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

        if output_handle == null_mut() {
            return Err("Couldn't retrieve the input handle.");
        }

        let window_handle = unsafe { GetConsoleWindow() };

        if window_handle == null_mut() {
            return Err("Couldn't retrieve the window handle.");
        }

        Ok(WindowsTerminal {
            console_handle,
            output_handle,
            input_handle,
            window_handle,
        })
    }
}

#[allow(dead_code)]
impl Terminal for WindowsTerminal {
    fn dispose(&self) -> Result<()> {
        unsafe { CloseHandle(self.console_handle) };
        Ok(())
    }

    fn set_cursor_visibility(&self, visible: bool) -> Result<()> {
        let mut console_cursor_info = CONSOLE_CURSOR_INFO::empty();
        let success = unsafe { GetConsoleCursorInfo(self.output_handle, &mut console_cursor_info) };

        if success == 0 {
            return Err("Problems trying to obtain the console cursor info.");
        }

        console_cursor_info.bVisible = if visible { 1 } else { 0 };

        let success = unsafe { SetConsoleCursorInfo(self.output_handle, &mut console_cursor_info) };

        if success == 0 {
            return Err("Problems trying to set the console cursor info.");
        }

        Ok(())
    }

    fn set_cursor(&self, position: Point2d) -> Result<()> {
        let success: i32 = unsafe {
            SetConsoleCursorPosition(
                self.output_handle,
                COORD {
                    X: position.x as i16,
                    Y: position.y as i16,
                },
            )
        };

        if success == 0 {
            return Err("Couldn't set the console cursor position.");
        }

        Ok(())
    }

    fn get_console_size(&self) -> Result<Size2d> {
        let mut console_screen_buffer_info = CONSOLE_SCREEN_BUFFER_INFO::empty();
        let success = unsafe {
            GetConsoleScreenBufferInfo(self.output_handle, &mut console_screen_buffer_info)
        };

        if success == 0 {
            return Err("Problems trying to obtain the screen buffer info.");
        }

        let window = console_screen_buffer_info.srWindow;

        Ok(Size2d::new(
            (window.Right - window.Left + 1) as usize,
            (window.Bottom - window.Top + 1) as usize,
        ))
    }

    fn get_char_size(&self, window: &Window) -> Result<Size2d> {
        let console_size = self.get_console_size()?;
        let client_size = window.get_window_client_size()?;

        if console_size.width == 0 || console_size.height == 0 {
            return Ok(Size2d::empty());
        }

        Ok(Size2d::new(
            client_size.width / console_size.width,
            client_size.height / console_size.height,
        ))
    }

    fn clear(&self) -> Result<()> {
        let size = self.get_console_size()?;
        let width = size.width as i16;
        let height = size.height as i16;
        let mut char_info = CHAR_INFO::empty();

        unsafe {
            *char_info.Char.UnicodeChar_mut() = ' ' as u16;
        }

        let char_info_array = vec![char_info; (width * height) as usize];

        let mut rect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: width as i16,
            Bottom: height as i16,
        };

        let success = unsafe {
            WriteConsoleOutputW(
                self.console_handle,
                char_info_array.as_ptr(),
                COORD {
                    X: width as i16,
                    Y: height as i16,
                },
                COORD::empty(),
                &mut rect as *mut SMALL_RECT,
            )
        };

        if success == 0 {
            return Err("Couldn't clear console output.");
        }

        Ok(())
    }

    fn write(&self, cell_buffer: &CellBuffer) -> Result<()> {
        // TODO: prevent from creating this char once per frame.
        //       each buffer should store his native representation
        //       and let the representation be updated when needed.
        let char_info_array = cell_buffer
            .iter()
            .map(|cell: &Cell| {
                let mut char_info = CHAR_INFO::empty();
                char_info.Attributes = get_u16_from_color(cell.foreground)
                    | (get_u16_from_color(cell.background) << 4);
                unsafe {
                    *char_info.Char.UnicodeChar_mut() = cell.character as u16;
                }
                char_info
            })
            .collect::<Vec<CHAR_INFO>>();

        let mut rect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: cell_buffer.size.width as i16,
            Bottom: cell_buffer.size.height as i16,
        };

        let success = unsafe {
            WriteConsoleOutputW(
                self.console_handle,
                char_info_array.as_ptr(),
                COORD {
                    X: cell_buffer.size.width as i16,
                    Y: cell_buffer.size.height as i16,
                },
                COORD::empty(),
                &mut rect as *mut SMALL_RECT,
            )
        };

        if success == 0 {
            return Err("Couldn't write to the console output.");
        }

        Ok(())
    }
}
