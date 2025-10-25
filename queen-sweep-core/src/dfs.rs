use std::collections::HashSet;

use crate::GameState;

pub fn depth_first_search(game_state: GameState) -> (Option<GameState>, usize) {
    let mut seen = HashSet::new();
    let mut steps = 0;
    let solution = dfs_helper(game_state, &mut seen, &mut steps);
    (solution, steps)
}

fn dfs_helper(
    game_state: GameState,
    seen: &mut HashSet<GameState>,
    steps: &mut usize,
) -> Option<GameState> {
    *steps += 1;

    if seen.contains(&game_state) {
        return None;
    }

    seen.insert(game_state.clone());

    if game_state.is_goal_state() {
        return Some(game_state);
    }

    for (r, c) in game_state.get_valid_placements() {
        let new_state = game_state.place_queen(r, c);
        if let Some(solution) = dfs_helper(new_state, seen, steps) {
            return Some(solution);
        }
    }

    None
}
