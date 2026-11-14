//! Color-region input and normalization.

use super::error::DynamicSolverError;
use crate::primitives::Color;

/// Color regions, supplied either as a square grid or as a flat row-major list.
///
/// The element type `T` may be [`Color`] itself or any type that converts into
/// [`Color`] (for example `usize`, via [`From<usize> for Color`](Color)).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionInput<T>
where
    T: Into<Color>,
{
    /// `grid[row][col]` is the color of that cell.
    Grid(Vec<Vec<T>>),
    /// Row-major list of `N*N` color values.
    Flat(Vec<T>),
}

impl<T> RegionInput<T>
where
    T: Into<Color>,
{
    /// Validates the input shape, converts each cell to a `usize` color index,
    /// and returns the board size together with a square `usize` grid.
    ///
    /// Color-range and color-count validation is deferred to
    /// [`QueensBoard::new`](crate::board::QueensBoard::new), which runs after
    /// the size is dispatched.
    pub(crate) fn normalize(self) -> Result<(usize, Vec<Vec<usize>>), DynamicSolverError> {
        match self {
            Self::Grid(grid) => {
                let rows = grid.len();
                if rows == 0 {
                    return Err(DynamicSolverError::Empty);
                }

                for row in &grid {
                    if row.len() != rows {
                        return Err(DynamicSolverError::NonSquare {
                            rows,
                            cols: row.len(),
                        });
                    }
                }

                let grid = grid
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|cell| usize::from(cell.into()))
                            .collect()
                    })
                    .collect();

                Ok((rows, grid))
            }
            Self::Flat(flat) => {
                let len = flat.len();
                if len == 0 {
                    return Err(DynamicSolverError::Empty);
                }

                let n = (len as f64).sqrt().round() as usize;
                if n * n != len {
                    return Err(DynamicSolverError::NotPerfectSquare(len));
                }

                let flat: Vec<usize> = flat
                    .into_iter()
                    .map(|cell| usize::from(cell.into()))
                    .collect();
                let grid = flat.chunks(n).map(|chunk| chunk.to_vec()).collect();

                Ok((n, grid))
            }
        }
    }
}
