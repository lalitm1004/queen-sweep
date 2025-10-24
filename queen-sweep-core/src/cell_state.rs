#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellState {
    Empty = 0,
    Blocked = 1,
    Queen = 2,
}

impl From<u8> for CellState {
    fn from(value: u8) -> Self {
        match value {
            0 => CellState::Empty,
            1 => CellState::Blocked,
            2 => CellState::Queen,
            _ => panic!("Invalid cell state value"),
        }
    }
}

impl From<CellState> for u8 {
    fn from(state: CellState) -> Self {
        state as u8
    }
}
