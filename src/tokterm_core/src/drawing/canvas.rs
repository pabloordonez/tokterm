use drawing::cell::Cell;
use drawing::cell_buffer::CellBuffer;
use drawing::point_2d::Point2d;
use drawing::size_2d::Size2d;
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

        let mut x0 = self.position.x as i32;
        let mut y0 = self.position.y as i32;
        let x1 = position.x as i32;
        let y1 = position.y as i32;
        let dx: i32 = (x1 - x0).abs();
        let dy: i32 = -(y1 - y0).abs();
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err: i32 = dx + dy;
        let mut e2: i32;

        while x0 != x1 || y0 != y1 {
            stroke.paint(
                &mut self.cell_buffer,
                Point2d::new(x0 as usize, y0 as usize),
            );

            e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x0 += sx;
            }

            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }

        self.move_to(position);
        Ok(())
    }

    pub fn bezier_to(&mut self, position: Point2d, control_point: Point2d) -> Result<()> {
        let stroke = match self.stroke {
            Some(stroke) => stroke,
            None => return Err("Can not draw a bezier line without a stroke."),
        };

        let mut x0 = self.position.x as f64;
        let mut y0 = self.position.y as f64;
        let mut x1 = position.x as f64;
        let mut y1 = position.y as f64;
        let mut cx = control_point.x as f64;
        let mut cy = control_point.y as f64;

        let mut x = x0 - cx;
        let mut y = y0 - cy;
        let mut t = x0 - 2.0 * cx + x1;
        let mut r: f64;

        if x * (x1 - cx) > 0.0 {
            if y * (y1 - cy) > 0.0 {
                if ((y0 - 2.0 * cy + y1) / t * x).abs() > y.abs() {
                    x0 = x1;
                    x1 = x + cx;
                    y0 = y1;
                    y1 = y + cy;
                }
            }

            t = (x0 - cx) / t;
            r = (1.0 - t) * ((1.0 - t) * y0 + 2.0 * t * cy) + t * t * y1;
            t = (x0 * x1 - cx * cx) * t / (x0 - cx);
            x = (t + 0.5).floor();
            y = (r + 0.5).floor();
            r = (cy - y0) * (t - x0) / (cx - x0) + y0;

            self.bezier_segment(
                x0 as i32,
                y0 as i32,
                x as i32,
                y as i32,
                x as i32,
                (r + 0.5).floor() as i32,
                stroke,
            )?;

            r = (cy - y1) * (t - x1) / (cx - x1) + y1;
            cx = x;
            x0 = cx;
            y0 = y;
            cy = (r + 0.5).floor();
        }
        if (y0 - cy) * (y1 - cy) > 0.0 {
            t = y0 - 2.0 * cy + y1;
            t = (y0 - cy) / t;
            r = (1.0 - t) * ((1.0 - t) * x0 + 2.0 * t * cx) + t * t * x1;
            t = (y0 * y1 - cy * cy) * t / (y0 - cy);
            x = (r + 0.5).floor();
            y = (t + 0.5).floor();
            r = (cx - x0) * (t - y0) / (cy - y0) + x0;

            self.bezier_segment(
                x0 as i32,
                y0 as i32,
                x as i32,
                y as i32,
                (r + 0.5).floor() as i32,
                y as i32,
                stroke,
            )?;

            r = (cx - x1) * (t - y1) / (cy - y1) + x1;
            x0 = x;
            cx = (r + 0.5).floor();
            cy = y;
            y0 = cy;
        }

        self.bezier_segment(
            x0 as i32, y0 as i32, x1 as i32, y1 as i32, cx as i32, cy as i32, stroke,
        )?;
        self.move_to(position);
        Ok(())
    }

    fn bezier_segment(
        &mut self,
        mut x0: i32,
        mut y0: i32,
        mut x1: i32,
        mut y1: i32,
        cx: i32,
        cy: i32,
        stroke: &Paint,
    ) -> Result<()> {
        let mut sx = x1 - cx;
        let mut sy = y1 - cy;
        let mut xy: i32;

        let mut xx = x0 - cx;
        let mut yy = y0 - cy;
        let mut dx: f64;
        let mut dy: f64;
        let mut err: f64;
        let mut curvature = (xx * sy - yy * sx) as f64;

        assert!(xx * sx <= 0 && yy * sy <= 0);

        if sx * sx + sy * sy > xx * xx + yy * yy {
            x1 = x0;
            x0 = sx + cx;
            y1 = y0;
            y0 = sy + cy;
            curvature = -curvature;
        }

        if curvature != 0.0 {
            xx += sx;
            sx = if x0 < x1 { 1 } else { -1 };
            xx *= sx;
            yy += sy;
            sy = if y0 < y1 { 1 } else { -1 };
            yy *= sy;
            xy = 2 * xx * yy;
            xx *= xx;
            yy *= yy;

            if (curvature * (sx * sy) as f64) < 0.0 {
                xx = -xx;
                yy = -yy;
                xy = -xy;
                curvature = -curvature;
            }

            dx = 4.0 * sy as f64 * curvature * (cx - x0) as f64 + (xx - xy) as f64;
            dy = 4.0 * sx as f64 * curvature * (y0 - cy) as f64 + (yy - xy) as f64;
            xx += xx;
            yy += yy;
            err = dx + dy + xy as f64;

            while dy <= dx {
                stroke.paint(self.cell_buffer, Point2d::new(x0 as usize, y0 as usize));

                if x0 == x1 && y0 == y1 {
                    return Ok(());
                }

                if 2.0 * err > dy {
                    x0 += sx;
                    dx -= xy as f64;
                    dy += yy as f64;
                    err += dy;
                }

                if 2.0 * err < dx {
                    y0 += sy;
                    dy -= xy as f64;
                    dx += xx as f64;
                    err += dx;
                }
            }
        }

        self.move_to(Point2d::new(x0 as usize, y0 as usize));
        self.line_to(Point2d::new(x1 as usize, y1 as usize))?;
        Ok(())
    }

    pub fn stroke_rect(&mut self, position: Point2d, size: Size2d) -> Result<()> {
        match self.stroke {
            None => return Err("Can not draw a rectangle without a stroke."),
            _ => (),
        };

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
            None => return Err("Can not fill a rectangle without a fill."),
        };

        let x0 = position.x;
        let y0 = position.y;
        let x1 = position.x + size.width;
        let y1 = position.y + size.height;

        for y in y0..=y1 {
            for x in x0..=x1 {
                fill.paint(&mut self.cell_buffer, Point2d::new(x, y));
            }
        }

        Ok(())
    }

    pub fn stroke_circle(&mut self, position: Point2d, radius: usize) -> Result<()> {
        return self.stroke_circle_from_center(position.add(Point2d::new(radius, radius)), radius);
    }

    pub fn stroke_circle_from_center(&mut self, center: Point2d, radius: usize) -> Result<()> {
        let stroke = match self.stroke {
            Some(stroke) => stroke,
            None => return Err("Can not draw a circle without a stroke."),
        };

        let cx = center.x;
        let cy = center.y;
        let mut r: i32 = radius as i32;
        let mut x: i32 = -r;
        let mut y: i32 = 0;
        let mut err: i32 = 2 - 2 * r;

        while x <= 0 {
            let ux = x as usize;
            let uy = y as usize;

            stroke.paint(&mut self.cell_buffer, Point2d::new(cx - ux, cy + uy));
            stroke.paint(&mut self.cell_buffer, Point2d::new(cx - uy, cy - ux));
            stroke.paint(&mut self.cell_buffer, Point2d::new(cx + ux, cy - uy));
            stroke.paint(&mut self.cell_buffer, Point2d::new(cx + uy, cy + ux));

            r = err;

            if r <= y {
                y += 1;
                err += y * 2 + 1;
            }

            if r > x || err > y {
                x += 1;
                err += x * 2 + 1;
            }
        }

        Ok(())
    }

    pub fn fill_circle(&mut self, position: Point2d, radius: usize) -> Result<()> {
        return self.fill_circle_from_center(position.add(Point2d::new(radius, radius)), radius);
    }

    pub fn fill_circle_from_center(&mut self, center: Point2d, radius: usize) -> Result<()> {
        let fill = match self.fill {
            Some(fill) => fill,
            None => return Err("Can not fill a circle without a fill."),
        };

        let cx = center.x;
        let cy = center.y;
        let mut r: i32 = radius as i32;
        let mut x: i32 = -r;
        let mut y: i32 = 0;
        let mut err: i32 = 2 - 2 * r;

        while x <= 0 {
            let ux = x as usize;
            for uy in cy - y as usize..=cy + y as usize {
                fill.paint(&mut self.cell_buffer, Point2d::new(cx + ux, uy));
                fill.paint(&mut self.cell_buffer, Point2d::new(cx - ux, uy));
            }

            r = err;

            if r <= y {
                y += 1;
                err += y * 2 + 1;
            }

            if r > x || err > y {
                x += 1;
                err += x * 2 + 1;
            }
        }

        Ok(())
    }

    pub fn stroke_ellipse_from_center(&mut self, center: Point2d, size: Size2d) -> Result<()> {
        return self.stroke_ellipse(
            center.sub(Point2d::new(size.width / 2, size.height / 2)),
            size,
        );
    }

    pub fn stroke_ellipse(&mut self, position: Point2d, size: Size2d) -> Result<()> {
        let stroke = match self.stroke {
            Some(stroke) => stroke,
            None => return Err("Can not draw an ellipse without a stroke."),
        };

        let mut a = size.width as i32;
        let b = size.height as i32;
        let mut b1 = b & 1;
        let mut x0 = position.x as i32;
        let mut y0 = position.y as i32;
        let mut x1 = position.x as i32 + a;
        let mut y1;

        let mut dx = 4 * (1 - a) * b * b;
        let mut dy = 4 * (b1 + 1) * a * a;
        let mut err = dx + dy + b1 * a * a;
        let mut e2;

        y0 += (b + 1) / 2;
        y1 = y0 - b1; /* starting pixel */
        a *= 8 * a;
        b1 = 8 * b * b;

        while x0 <= x1 {
            let ux0 = x0 as usize;
            let uy0 = y0 as usize;
            let ux1 = x1 as usize;
            let uy1 = y1 as usize;

            stroke.paint(&mut self.cell_buffer, Point2d::new(ux0, uy0));
            stroke.paint(&mut self.cell_buffer, Point2d::new(ux0, uy1));
            stroke.paint(&mut self.cell_buffer, Point2d::new(ux1, uy0));
            stroke.paint(&mut self.cell_buffer, Point2d::new(ux1, uy1));

            e2 = 2 * err;

            if e2 <= dy {
                y0 += 1;
                y1 -= 1;
                dy += a;
                err += dy;
            }

            if e2 >= dx || 2 * err > dy {
                x0 += 1;
                x1 -= 1;
                dx += b1;
                err += dx;
            }
        }

        let ux0 = x0 as usize;
        let ux1 = x1 as usize;

        while y0 - y1 < b {
            let uy0 = y0 as usize;
            let uy1 = y1 as usize;

            stroke.paint(&mut self.cell_buffer, Point2d::new(ux0, uy0));
            stroke.paint(&mut self.cell_buffer, Point2d::new(ux0, uy1));
            stroke.paint(&mut self.cell_buffer, Point2d::new(ux1, uy0));
            stroke.paint(&mut self.cell_buffer, Point2d::new(ux1, uy1));

            y0 += 1;
            y1 -= 1;
        }

        Ok(())
    }

    pub fn fill_ellipse_from_center(&mut self, center: Point2d, size: Size2d) -> Result<()> {
        return self.fill_ellipse(
            center.sub(Point2d::new(size.width / 2, size.height / 2)),
            size,
        );
    }

    pub fn fill_ellipse(&mut self, position: Point2d, size: Size2d) -> Result<()> {
        let fill = match self.fill {
            Some(fill) => fill,
            None => return Err("Can not fill an ellipse without a fill."),
        };

        let mut a = size.width as i32;
        let b = size.height as i32;
        let mut b1 = b & 1;
        let mut x0 = position.x as i32;
        let mut y0 = position.y as i32;
        let mut x1 = position.x as i32 + a;
        let mut y1;

        let mut dx = 4 * (1 - a) * b * b;
        let mut dy = 4 * (b1 + 1) * a * a;
        let mut err = dx + dy + b1 * a * a;
        let mut e2;

        y0 += (b + 1) / 2;
        y1 = y0 - b1; /* starting pixel */
        a *= 8 * a;
        b1 = 8 * b * b;

        while x0 <= x1 {
            let ux0 = x0 as usize;
            let uy0 = y0 as usize;
            let ux1 = x1 as usize;
            let uy1 = y1 as usize;

            for ux in ux0..=ux1 {
                fill.paint(&mut self.cell_buffer, Point2d::new(ux, uy0));
                fill.paint(&mut self.cell_buffer, Point2d::new(ux, uy1));
            }

            e2 = 2 * err;

            if e2 <= dy {
                y0 += 1;
                y1 -= 1;
                dy += a;
                err += dy;
            }

            if e2 >= dx || 2 * err > dy {
                x0 += 1;
                x1 -= 1;
                dx += b1;
                err += dx;
            }
        }

        let ux0 = x0 as usize;
        let ux1 = x1 as usize;

        while y0 - y1 < b {
            let uy0 = y0 as usize;
            let uy1 = y1 as usize;

            for uy in uy0..=uy1 {
                fill.paint(&mut self.cell_buffer, Point2d::new(ux0, uy));
                fill.paint(&mut self.cell_buffer, Point2d::new(ux1, uy));
            }

            y0 += 1;
            y1 -= 1;
        }

        Ok(())
    }
}
