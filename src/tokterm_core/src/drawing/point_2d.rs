#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Point2d {
    pub fn new(x: i32, y: i32) -> Point2d {
        Point2d { x, y }
    }

    pub fn empty() -> Point2d {
        Point2d { x: 0, y: 0 }
    }

    pub fn add(&self, point: Point2d) -> Point2d {
        Point2d::new(self.x + point.x, self.y + point.y)
    }

    pub fn add_x(&self, x: i32) -> Point2d {
        Point2d::new(self.x + x, self.y)
    }

    pub fn add_y(&self, y: i32) -> Point2d {
        Point2d::new(self.x, self.y + y)
    }

    pub fn equal_to(&self, point: Point2d) -> bool {
        self.x == point.x && self.y == point.y
    }

    pub fn is_empty(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}
