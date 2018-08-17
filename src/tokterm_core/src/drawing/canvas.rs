use drawing::cell::Cell;
use drawing::cell_buffer::CellBuffer;
use drawing::point_2d::Point2d;
use drawing::size_2d::Size2d;
use std::mem;
use Result;

pub trait Paint {
    fn paint(&self, cell_buffer: &mut CellBuffer, position: Point2d);
}

pub struct SolidPaint {
    cell: Cell,
}

impl SolidPaint {
    pub fn new(cell: Cell) -> SolidPaint {
        SolidPaint { cell }
    }
}

impl Paint for SolidPaint {
    #[inline]
    fn paint(&self, cell_buffer: &mut CellBuffer, position: Point2d) {
        cell_buffer.set(position, self.cell);
    }
}

pub struct Canvas<'a> {
    cell_buffer: &'a mut CellBuffer,
    position: Point2d,
    stroke: Option<&'a Paint>,
    fill: Option<&'a Paint>,
}

impl<'a> Canvas<'a> {
    pub fn new(
        cell_buffer: &'a mut CellBuffer,
        stroke: Option<&'a Paint>,
        fill: Option<&'a Paint>,
    ) -> Canvas<'a> {
        Canvas::<'a> {
            cell_buffer,
            position: Point2d::empty(),
            stroke: stroke,
            fill: fill,
        }
    }

    #[inline]
    pub fn set_stroke(&mut self, painter: &'a Paint) {
        self.stroke = Some(painter);
    }

    #[inline]
    pub fn set_fill(&mut self, painter: &'a Paint) {
        self.fill = Some(painter);
    }

    #[inline]
    pub fn move_to(&mut self, position: Point2d) {
        self.position = position;
    }

    pub fn line_to(&mut self, position: Point2d) -> Result<()> {
        let stroke = match self.stroke {
            Some(stroke) => stroke,
            None => return Err("Can not draw a line without a stroke."),
        };

        let mut steep = false;
        let mut x0 = self.position.x as i32;
        let mut y0 = self.position.y as i32;
        let mut x1 = position.x as i32;
        let mut y1 = position.y as i32;

        if (x0 - x1).abs() < (y0 - y1).abs() {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            steep = true;
        }
        if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let d_error2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = y0;
        let y_inc = if y1 > y0 { 1 } else { -1 };

        for x in x0..x1 + 1 {
            let position = if steep {
                Point2d::new(y as usize, x as usize)
            } else {
                Point2d::new(x as usize, y as usize)
            };

            stroke.paint(&mut self.cell_buffer, position);
            error2 += d_error2;

            if error2 > dx {
                y += y_inc;
                error2 -= dx * 2;
            }
        }

        self.move_to(position);
        Ok(())
    }

    pub fn stroke_rect(&mut self, position: Point2d, size: Size2d) -> Result<()> {
        let x0 = position.x;
        let y0 = position.y;
        let x1 = position.x + size.width;
        let y1 = position.y + size.height;

        self.move_to(Point2d::new(x0, y0));
        self.line_to(Point2d::new(x1, y0))?;
        self.line_to(Point2d::new(x1, y1))?;
        self.line_to(Point2d::new(x0, y1))?;
        self.line_to(Point2d::new(x0, y0))?;

        Ok(())
    }

    pub fn fill_rect(&mut self, position: Point2d, size: Size2d) -> Result<()> {
        let fill = match self.fill {
            Some(fill) => fill,
            None => return Err("Can not draw a line without a stroke."),
        };

        let x0 = position.x;
        let y0 = position.y;
        let x1 = position.x + size.width;
        let y1 = position.y + size.height;

        for y in y0..y1 + 1 {
            for x in x0..x1 + 1 {
                fill.paint(&mut self.cell_buffer, Point2d::new(x, y));
            }
        }

        Ok(())
    }

    pub fn stroke_circle(&mut self, position: Point2d, radius: usize) -> Result<()> {
        let stroke = match self.stroke {
            Some(stroke) => stroke,
            None => return Err("Can not draw a circle without a stroke."),
        };

        let x0 = position.x;
        let y0 = position.y;
        let mut x = radius - 1;
        let mut y = 0;
        let mut dx = 1 as i32;
        let mut dy = 1 as i32;
        let mut err = dx - ((radius as i32) << 1);

        while x >= y {
            stroke.paint(&mut self.cell_buffer, Point2d::new(x0 + x, y0 + y));
            stroke.paint(&mut self.cell_buffer, Point2d::new(x0 + y, y0 + x));
            stroke.paint(&mut self.cell_buffer, Point2d::new(x0 - y, y0 + x));
            stroke.paint(&mut self.cell_buffer, Point2d::new(x0 - x, y0 + y));
            stroke.paint(&mut self.cell_buffer, Point2d::new(x0 - x, y0 - y));
            stroke.paint(&mut self.cell_buffer, Point2d::new(x0 - y, y0 - x));
            stroke.paint(&mut self.cell_buffer, Point2d::new(x0 + y, y0 - x));
            stroke.paint(&mut self.cell_buffer, Point2d::new(x0 + x, y0 - y));

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            }

            if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - ((radius as i32) << 1);
            }
        }

        Ok(())
    }

    pub fn fill_circle(&mut self, position: Point2d, radius: usize) -> Result<()> {
        let fill = match self.fill {
            Some(fill) => fill,
            None => return Err("Can not draw a line without a stroke."),
        };

        let x0 = position.x;
        let y0 = position.y;
        let mut x = radius - 1;
        let mut y = 0;
        let mut dx = 1 as i32;
        let mut dy = 1 as i32;
        let mut err = dx - ((radius as i32) << 1);

        while x >= y {
            for xp in x0 - x..x0 + x + 1 {
                fill.paint(&mut self.cell_buffer, Point2d::new(xp, y0 + y));
                fill.paint(&mut self.cell_buffer, Point2d::new(xp, y0 - y));
            }

            for xp in x0 - y..x0 + y + 1 {
                fill.paint(&mut self.cell_buffer, Point2d::new(xp, y0 + x));
                fill.paint(&mut self.cell_buffer, Point2d::new(xp, y0 - x));
            }

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            }

            if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - ((radius as i32) << 1);
            }
        }

        Ok(())
    }
}
