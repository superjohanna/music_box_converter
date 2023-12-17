pub mod functions;

#[derive(Clone, Debug)]
pub struct MetaInformation {
    // Length in miditicks
    pub length: u64,
    // Minimum delta between two notes; Yes, they are oversized but I don't want to use u28 or whatever
    pub min_distance: u32,
    // Maximum delta between two notes
    pub max_distance: u32,
}
