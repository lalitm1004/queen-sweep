#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    value: usize,
}

impl Color {
    #[inline]
    pub const fn value(&self) -> usize {
        self.value
    }
}

impl From<usize> for Color {
    #[inline]
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl From<Color> for usize {
    #[inline]
    fn from(color: Color) -> Self {
        color.value()
    }
}
