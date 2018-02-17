use level::Level;
use std::convert::TryFrom;

use path::Path;

#[derive(Debug)]
pub struct Mrg {
    pub levels: Vec<Level>,
}

impl TryFrom<Vec<u8>> for Mrg {
    type Error = String;

    fn try_from(content: Vec<u8>) -> Result<Mrg, Self::Error> {
        let mut levels: Vec<Level> = Vec::with_capacity(3);
        let mut offset = 0;
        for _ in 0..3 {
            let new_level = Level::try_from(&content[offset..]).expect(
                "Error when converting level from &[u8]",
            );
            offset += new_level.get_size();
            levels.push(new_level);
        }
        for level in &mut levels {
            for track in &mut level.tracks {
                let new_track = Path::try_from(&content[track.offset as usize..]).expect(
                    "Не удалось получить трэк по оффсету",
                );
                track.path = Some(new_track);
            }
        }
        Ok(Mrg { levels })
    }
}
