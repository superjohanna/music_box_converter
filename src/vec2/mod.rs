pub mod functions;

#[derive(Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Default> Vec2<T> {
    pub fn new() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
