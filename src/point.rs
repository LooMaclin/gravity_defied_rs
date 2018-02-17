use std::convert::TryFrom;
use ggez::graphics::Point2 as GgezPoint;



#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn get_size(&self) -> usize {
        2
    }
}

impl Into<GgezPoint> for Point {
    fn into(self) -> GgezPoint {
        GgezPoint::new(self.x as f32, self.y as f32)
    }
}

impl<'a> TryFrom<&'a [u8]> for Point {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Point {
            x: value[0] as i8,
            y: value[1] as i8,
        })
    }
}
