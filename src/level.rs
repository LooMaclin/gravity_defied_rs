use std::convert::TryFrom;
use track_name::TrackName;
use byteorder::{ByteOrder, BigEndian};

#[derive(Debug)]
pub struct Level {
    pub count: u32,
    pub tracks: Vec<TrackName>,
}

impl Level {
    pub fn get_size(&self) -> usize {
        4 +
            self.tracks.iter().fold(0, |mut acc, item| {
                acc += item.get_size();
                acc
            })
    }
}

impl<'a> TryFrom<&'a [u8]> for Level {
    type Error = ();

    fn try_from(content: &[u8]) -> Result<Self, Self::Error> {
        let count = BigEndian::read_u32(&content[..4]);
        let mut tracks = Vec::with_capacity(64);
        let mut offset: usize = 0;
        let content = &content[4..];
        for _ in 0..count {
            let new_track_name = TrackName::try_from(&content[offset..]).expect(
                "Error when converting TrackName from &[u8]",
            );
            offset += new_track_name.get_size();
            tracks.push(new_track_name);
        }
        Ok(Level { count, tracks })
    }
}
