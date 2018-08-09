#[derive(Debug, Copy, Clone)]
pub struct Size2d {
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl Size2d {
    pub fn new(width: usize, height: usize) -> Size2d {
        Size2d { width, height }
    }

    pub fn empty() -> Size2d {
        Size2d {
            width: 0,
            height: 0,
        }
    }

    pub fn add(&self, point: Size2d) -> Size2d {
        Size2d::new(self.width + point.width, self.height + point.height)
    }

    pub fn add_width(&self, width: usize) -> Size2d {
        Size2d::new(self.width + width, self.height)
    }

    pub fn add_height(&self, height: usize) -> Size2d {
        Size2d::new(self.width, self.height + height)
    }

    pub fn equal_to(&self, size: Size2d) -> bool {
        self.width == size.width && self.height == size.height
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 && self.height == 0
    }
}
