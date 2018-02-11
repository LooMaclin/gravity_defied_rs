use point::Point;
use std::convert::TryFrom;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use ggez::graphics::Point as GgezPoint;

const START_TRACK_DESCRIPTION_MARK: u8 = 0x33;

#[derive(Debug, Clone)]
pub struct Path {
    pub start_x: u32,
    pub start_y: u32,
    pub finish_x: u32,
    pub finish_y: u32,
    pub points_count: u16,
    pub point_x: i32,
    pub point_y: i32,
    pub points: Vec<Point>,
}

fn what_the_fuck_is_it(original_number: u32) -> u32 {
    (original_number << 16) >> 3
}

impl<'a> TryFrom<&'a [u8]> for Path {
    type Error = String;
    fn try_from(content: &[u8]) -> Result<Self, Self::Error> {
        if content[0] == START_TRACK_DESCRIPTION_MARK {
            let content = &content[1..];
            let start_x = what_the_fuck_is_it(BigEndian::read_u32(&content[..4]));
            let start_y = what_the_fuck_is_it(BigEndian::read_u32(&content[4..8]));
            let finish_x = what_the_fuck_is_it(BigEndian::read_u32(&content[8..12]));
            let finish_y = what_the_fuck_is_it(BigEndian::read_u32(&content[12..16]));
            let points_count = BigEndian::read_u16(&content[16..18]);
            let point_x = BigEndian::read_i32(&content[18..22]);
            let point_y = BigEndian::read_i32(&content[22..26]);
            let mut offset = 0;
            let content = &content[26..];
            let mut points = Vec::with_capacity(64);
            for i in 0..points_count - 1 {
                let point = Point::try_from(&content[offset..]).expect(
                    "Не удалось прочесть Point из файла",
                );
                offset += point.get_size();
                if point.x == -1 {
                    return Ok(Path {
                        start_x,
                        start_y,
                        finish_x,
                        finish_y,
                        points_count,
                        point_x,
                        point_y,
                        points,
                    });
                } else {
                    points.push(point);
                }
            }
            Ok(Path {
                start_x,
                start_y,
                finish_x,
                finish_y,
                points_count,
                point_x,
                point_y,
                points,
            })
        } else {
            Err("Маркер описания трэка в начале переданного слайса не обнаружен".to_owned())
        }
    }
}

impl Into<Vec<GgezPoint>> for Path {
    fn into(self) -> Vec<GgezPoint> {
        let mut new_points = vec![GgezPoint { x: 200.0, y: 200.0 }];
        for item in self.points.windows(2).enumerate() {
            let x = new_points[item.0].x as i32 + item.1[0].x as i32;
            let mut y = new_points[item.0].y as i32 - item.1[0].y as i32;
            new_points.push(GgezPoint::new(x as f32, y as f32));
        }
        new_points
    }
}
