use midly::MidiMessage;

use super::MetaInformation;

impl MetaInformation {
    pub fn gather_meta(track: &[midly::TrackEvent]) -> Self {
        let mut info: Self = Self {
            length: u64::MIN,
            min_distance_same_notes: u64::MAX,
            max_distance_same_notes: u64::MIN,
        };

        // 127 is the limit for the u7 type so we only have to worry about these 127 pitches
        let mut delta_for_pitches: [Option<u64>; 127 as usize] = [None; 127];

        for (i, event) in track.iter().enumerate() {
            // Why this? Because we only care about notes. Also they have to be the same note... Man that's expensive... And it could all be avoided if we used absolute instead of relative....            
            match event.kind {
                midly::TrackEventKind::Midi {
                    channel,
                    message: MidiMessage::NoteOn { key, .. },
                } => {
                    if u64::from(u32::from(event.delta)) < info.min_distance_same_notes && delta_for_pitches[] {
                        info.min_distance_same_notes = u64::from(u32::from(event.delta))
                    }
                    if u64::from(u32::from(event.delta)) > info.max_distance_same_notes {
                        info.max_distance_same_notes = u64::from(u32::from(event.delta))
                    }

                }
                _ => (),
            }

            // Add the current delta to all pitches. If you don't understand why I'm sorry. I barely understand it myself
            delta_for_pitches
                .iter_mut()
                .for_each(|el| {
                        if el.is_some() {
                            *el = Some(u64::from(u32::from(event.delta)) + el.unwrap())
                        }
                    }
                );
            info.length += u64::from(u32::from(event.delta));
        }

        info
    }
}
