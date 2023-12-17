use super::MetaInformation;

impl MetaInformation {
    pub fn gather_meta(track: &Vec<midly::TrackEvent>) -> Self {
        let mut info: Self = Self {
            length: u64::MIN,
            min_distance: u32::MAX,
            max_distance: u32::MIN,
        };

        for (i, event) in track.iter().enumerate() {
            if event.delta < info.min_distance {
                info.min_distance = u32::from(event.delta)
            }
            if event.delta > info.max_distance {
                info.max_distance = u32::from(event.delta)
            }
            info.length += u64::from(u32::from(event.delta));
        }

        info
    }
}
