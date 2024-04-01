use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item(String, String);
pub type List = Vec<Item>;
pub const PATH: &str = "../../lang/";
fn main() {
    // Did you back up your files? This program can panic and it overwrites files.
    // You need to launch this with 'cargo run -- yes
    if !std::env::args().any(|x| x.contains("yes")) {
        std::process::exit(1);
    }
    let iter = std::fs::read_dir(PATH).unwrap();
    for entry in iter {
        let unwrapped = entry.unwrap();
        let file = std::fs::File::open(unwrapped.path()).unwrap();

        let mut list: List = serde_json::from_reader(file).unwrap();

        // This list comes from a hashmap so it can't contain the same key two times. unstable is fine
        list.sort_unstable_by(|x, y| x.0.cmp(&y.0));

        let file = std::fs::File::create(unwrapped.path()).unwrap();

        let _ = serde_json::to_writer_pretty(file, &list);
    }
    std::process::exit(0);
}
