use std::collections::HashSet;

use crate::GameState;

pub fn depth_first_search(game_state: GameState) -> Option<GameState> {
    let mut seen = HashSet::new();
    dfs_helper(game_state, &mut seen)
}

fn dfs_helper(game_state: GameState, seen: &mut HashSet<GameState>) -> Option<GameState> {
    if seen.contains(&game_state) {
        return None;
    }

    seen.insert(game_state.clone());

    if game_state.is_goal_state() {
        return Some(game_state);
    }

    for (r, c) in game_state.get_valid_queen_placements() {
        let new_state = game_state.place_queen(r, c);
        let solution = dfs_helper(new_state, seen);
        if solution.is_some() {
            return solution;
        }
    }

    None
}
