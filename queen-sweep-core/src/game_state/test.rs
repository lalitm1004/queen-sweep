use crate::{GameState, GameStateError, depth_first_search, game_state::MAX_BOARD_SIZE, heuristic};

fn puzzle_8x8_sol() -> Vec<Vec<u8>> {
    // Solution Exists
    vec![
        vec![0, 0, 1, 1, 1, 2, 2, 2],
        vec![0, 3, 1, 3, 1, 4, 2, 2],
        vec![0, 3, 1, 3, 1, 2, 2, 2],
        vec![0, 3, 3, 3, 1, 5, 6, 2],
        vec![0, 3, 3, 3, 1, 5, 6, 6],
        vec![0, 3, 7, 3, 1, 5, 6, 6],
        vec![7, 3, 7, 3, 1, 5, 5, 6],
        vec![7, 7, 7, 7, 6, 6, 6, 6],
    ]
}

fn puzzle_3x3_nosol() -> Vec<Vec<u8>> {
    vec![vec![0, 0, 1], vec![1, 1, 2], vec![2, 2, 2]]
}

fn get_initial_game_state_8x8() -> GameState {
    GameState::from_color_regions(
        puzzle_8x8_sol(),
        Some(heuristic::smallest_region_by_empty_cells),
    )
    .unwrap()
}

#[test]
fn test_valid_board_init() {
    assert!(GameState::from_color_regions(puzzle_3x3_nosol(), None).is_ok());
    assert!(GameState::from_color_regions(puzzle_8x8_sol(), None).is_ok());
}

#[test]
fn test_gamestate_empty_board() {
    let puzzle: Vec<Vec<u8>> = vec![];
    let state = GameState::try_from(puzzle);

    assert!(state.is_err());
    assert!(matches!(
        state.unwrap_err(),
        GameStateError::InexistentBoard
    ));
}

#[test]
fn test_gamestate_non_square_board() {
    let puzzle = vec![vec![0, 1, 2], vec![0, 1]];
    let state = GameState::try_from(puzzle);

    assert!(state.is_err());
    assert!(matches!(
        state.unwrap_err(),
        GameStateError::NonSquareBoard { .. }
    ));
}

#[test]
fn test_gamestate_board_too_large() {
    let size = MAX_BOARD_SIZE + 1;
    let puzzle = vec![vec![0; size as usize]; size as usize];
    let state = GameState::try_from(puzzle);

    assert!(state.is_err());
    assert!(matches!(
        state.unwrap_err(),
        GameStateError::BoardTooLarge { .. }
    ));
}

#[test]
fn test_can_place_queen() {
    let state = get_initial_game_state_8x8();

    // invalid moves
    assert!(!state.can_place_queen(state.states(), state.colors_with_queens(), 0, 5));
    assert!(!state.can_place_queen(state.states(), state.colors_with_queens(), 1, 1));
    assert!(!state.can_place_queen(state.states(), state.colors_with_queens(), 4, 5));

    // valid moves
    assert!(state.can_place_queen(state.states(), state.colors_with_queens(), 1, 5));
    assert!(state.can_place_queen(state.states(), state.colors_with_queens(), 7, 7));
    assert!(state.can_place_queen(state.states(), state.colors_with_queens(), 1, 5));

    let state = state.place_queen(1, 5);

    // invalid moves
    assert!(!state.can_place_queen(state.states(), state.colors_with_queens(), 4, 7));
    assert!(!state.can_place_queen(state.states(), state.colors_with_queens(), 7, 6));
    assert!(!state.can_place_queen(state.states(), state.colors_with_queens(), 6, 0));

    // valid moves
    assert!(state.can_place_queen(state.states(), state.colors_with_queens(), 0, 0));
    assert!(state.can_place_queen(state.states(), state.colors_with_queens(), 5, 2));
    assert!(state.can_place_queen(state.states(), state.colors_with_queens(), 6, 6));
}

#[test]
fn test_8x8_has_solution() {
    let state = get_initial_game_state_8x8();

    let (solved_state, _) = depth_first_search(state);
    assert!(solved_state.is_some());
}

#[test]
fn test_3x3_has_no_solution() {
    let state = GameState::from_color_regions(puzzle_3x3_nosol(), None).unwrap();

    let (solved_state, _) = depth_first_search(state);
    assert!(solved_state.is_none());
}
