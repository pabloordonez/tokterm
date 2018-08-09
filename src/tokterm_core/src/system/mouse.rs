use drawing::point_2d::Point2d;
use Result;

pub trait Mouse {
    /// Gets the absolute mouse position.
    fn get_absolute_position(&self) -> Result<Point2d>;

    /// Gets the mouse position in relation to the client window.
    fn get_client_position(&self) -> Result<Point2d>;

    /// Sets the mouse position.
    fn set_position(&self, position: Point2d) -> Result<()>;

    /// Shows or hides the mouse cursor.
    fn show_cursor(&self, visible: bool) -> Result<()>;
}
