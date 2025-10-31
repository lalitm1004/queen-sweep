use std::collections::HashSet;

use crate::GameState;

pub fn depth_first_search(game_state: GameState) -> (Option<GameState>, usize) {
    let mut seen = HashSet::new();
    let mut states_visited = 0;
    let solution = dfs_helper_with_counter(game_state, &mut seen, &mut states_visited);
    (solution, states_visited)
}

fn dfs_helper_with_counter(
    game_state: GameState,
    seen: &mut HashSet<GameState>,
    states_visited: &mut usize,
) -> Option<GameState> {
    *states_visited += 1;

    if seen.contains(&game_state) {
        return None;
    }

    seen.insert(game_state.clone());

    if game_state.is_goal_state() {
        return Some(game_state);
    }

    for (r, c) in game_state.valid_placements() {
        let new_state = game_state.place_queen(r, c);
        if let Some(solution) = dfs_helper_with_counter(new_state, seen, states_visited) {
            return Some(solution);
        }
    }

    None
}
