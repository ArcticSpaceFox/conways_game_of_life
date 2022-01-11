/// Point defines a location in a 2d space
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(item: (usize, usize)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

impl From<(&usize, &usize)> for Point {
    fn from(item: (&usize, &usize)) -> Self {
        Self {
            x: item.0.to_owned(),
            y: item.1.to_owned(),
        }
    }
}
