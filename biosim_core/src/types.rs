pub type Dimension = u16;

#[derive(Debug)]
pub struct Coord {
    pub x: Dimension,
    pub y: Dimension,
}

#[derive(Debug, PartialEq)]
pub enum RunMode {
    RUN,
    STOP,
    PAUSE,
    ABORT,
}
