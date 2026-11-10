/// Identifies one of the `N` colored regions of a board.
///
/// A color is represented by its zero-based index (`0..N`). Instances are
/// created from `usize` via [`From`] and converted back to `usize` via
/// [`Into`].
///
/// [`Color`] is a thin wrapper that performs no validation: it is the
/// responsibility of [`QueensBoard::new_unchecked`](crate::board::QueensBoard::new_unchecked)
/// to guarantee that every color index is strictly less than the board size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    value: usize,
}

impl Color {
    /// Returns the zero-based color index.
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
