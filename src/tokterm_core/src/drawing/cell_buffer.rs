use drawing::cell::Cell;
use drawing::color::Color;
use drawing::point_2d::Point2d;
use drawing::size_2d::Size2d;
use std::slice::Iter;
use std::str::Chars;

#[derive(Debug)]
pub struct CellBuffer {
    pub size: Size2d,
    cells: Vec<Cell>,
}

#[allow(dead_code)]
impl CellBuffer {
    pub fn new(default_cell: Cell, size: Size2d) -> CellBuffer {
        CellBuffer {
            size,
            cells: vec![default_cell; size.width * size.height],
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Cell> {
        self.cells.iter()
    }

    pub fn resize(&mut self, default_cell: Cell, new_size: Size2d) {
        if new_size == self.size {
            return;
        }

        self.size = new_size;
        self.cells = vec![default_cell; new_size.width * new_size.height];
    }

    #[inline]
    pub fn index_of(&self, position: Point2d) -> Option<usize> {
        let ux = position.x as usize;
        let uy = position.y as usize;

        if position.x < 0 || position.y < 0 || ux >= self.size.width || uy >= self.size.height {
            return Option::None;
        }

        let index = ux + self.size.width * uy;

        if index >= self.cells.len() {
            return Option::None;
        }

        Option::Some(index)
    }

    #[inline]
    pub fn coordinates_of(&self, index: usize) -> Option<Point2d> {
        if self.size.width == 0 || self.size.height == 0 {
            return None;
        }

        Option::Some(Point2d::new(
            (index % self.size.width) as i32,
            (index / self.size.width) as i32,
        ))
    }

    #[inline]
    pub fn get(&self, position: Point2d) -> Option<Cell> {
        return match self.index_of(position) {
            Some(index) => Some(self.cells[index]),
            None => Option::None,
        };
    }

    #[inline]
    pub fn set(&mut self, position: Point2d, cell: Cell) {
        let index = match self.index_of(position) {
            Some(index) => index,
            None => return,
        };

        self.cells[index] = cell;
    }

    pub fn write_chars(
        &mut self,
        text: Chars,
        position: Point2d,
        foreground: Color,
        background: Color,
    ) {
        let mut buffer_index = match self.index_of(position) {
            Some(index) => index,
            None => return,
        };

        for character in text {
            self.cells[buffer_index].character = character;
            self.cells[buffer_index].foreground = foreground;
            self.cells[buffer_index].background = background;

            buffer_index += 1;

            if buffer_index >= self.cells.len() {
                return;
            }
        }
    }

    pub fn write_str(
        &mut self,
        text: &str,
        position: Point2d,
        foreground: Color,
        background: Color,
    ) {
        self.write_chars(text.chars(), position, foreground, background);
    }

    pub fn repeat_cell(&mut self, cell: Cell, position: Point2d, length: usize) {
        let buffer_index = match self.index_of(position) {
            Some(index) => index,
            None => return,
        };

        for index in 0..length {
            if buffer_index + index >= self.cells.len() {
                return;
            }

            self.cells[buffer_index + index] = cell;
        }
    }

    pub fn write_cell_buffer(&mut self, cell_buffer: &CellBuffer, position: Point2d) {
        let width = cell_buffer.size.width as i32;
        let height = cell_buffer.size.height as i32;
        let self_width = self.size.width as i32;
        let self_height = self.size.height as i32;

        for cby in 0..height {
            let destination_y: i32 = position.y + cby;

            if destination_y >= self_height {
                break;
            }

            for cbx in 0..width {
                let destination_x: i32 = position.x + cbx;

                if destination_x >= self_width {
                    break;
                }

                let cell = match cell_buffer.get(Point2d::new(cbx, cby)) {
                    Some(cell) => cell,
                    None => return,
                };

                self.set(Point2d::new(destination_x, destination_y), cell);
            }
        }
    }
}
