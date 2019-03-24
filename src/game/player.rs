pub struct Player {
    pub pos: (f64, f64),
}
impl Player {
    pub fn new() -> Self {
        Player { pos: (39.5, 39.5) }
    }
}
