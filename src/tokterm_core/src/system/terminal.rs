use drawing::cell_buffer::CellBuffer;
use drawing::point_2d::Point2d;
use drawing::size_2d::Size2d;
use Result;

pub trait Terminal {
    /// Shows or hides the cursor.
    fn set_cursor_visibility(&mut self, visible: bool) -> Result<()>;

    /// Moves the console cursor to a given position.
    fn set_cursor(&mut self, position: Point2d) -> Result<()>;

    /// Gets the current console size in character units.
    fn get_console_size(&self) -> Result<Size2d>;

    /// Clears the console screen.
    fn clear(&mut self) -> Result<()>;

    /// Draws a `CellBuffer` to the screen.
    fn write(&mut self, cell_buffer: &mut CellBuffer) -> Result<()>;
}
