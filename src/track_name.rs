use std::convert::TryFrom;
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use path::Path;

#[derive(Debug)]
pub struct TrackName {
    pub offset: u32,
    pub name: String,
    pub path: Option<Path>,
}

impl TrackName {
    pub fn get_size(&self) -> usize {
        4 + self.name.len() + 1
    }
}

impl<'a> TryFrom<&'a [u8]> for TrackName {
    type Error = ();

    fn try_from(content: &[u8]) -> Result<Self, Self::Error> {
        let offset = BigEndian::read_u32(&content[..4]);
        let content = &content[4..];
        let (position, _) = content.iter().enumerate().find(|&(position, item)| {
            *item == 0
        }).expect("Строка с именем уровня в файле не терминируется");
        let mut name: String = String::from_utf8_lossy(&content[..position]).to_string();
        Ok(TrackName {
            offset,
            name,
            path: None,
        })
    }
}
