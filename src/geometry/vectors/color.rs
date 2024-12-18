use std::fmt::Display;

#[allow(unused)]
#[derive(Clone, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(unused)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}