use super::*;

fn get_initial_game_state() -> GameState {
    let color_regions = vec![
        vec![0, 0, 1, 1, 1, 2, 2, 2],
        vec![0, 3, 1, 3, 1, 4, 2, 2],
        vec![0, 3, 1, 3, 1, 2, 2, 2],
        vec![0, 3, 3, 3, 1, 5, 6, 2],
        vec![0, 3, 3, 3, 1, 5, 6, 6],
        vec![0, 3, 7, 3, 1, 5, 6, 6],
        vec![7, 3, 7, 3, 1, 5, 5, 6],
        vec![7, 7, 7, 7, 6, 6, 6, 6],
    ];
    GameState::from_color_regions(color_regions, None)
        .expect("should be able to initialize game state")
}

#[test]
fn test_can_place_queen_blocking() {
    let game_state = get_initial_game_state();
    let size = game_state.size;
    let r: usize = 1;
    let c: usize = 5;

    for i in 0..size {
        // cant place anywhere in row
        if i != c {
            assert!(!game_state.can_place_queen(r, i));
        }

        // cant place anywhere in col
        if i != r {
            assert!(!game_state.can_place_queen(i, c));
        }
    }

    // cant place in neighborhood
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < size as i32 && nc >= 0 && nc < size as i32 {
                assert!(!game_state.can_place_queen(nr as usize, nc as usize))
            }
        }
    }
}

#[test]
fn test_can_place_queen_non_blocking() {
    let game_state = get_initial_game_state();
    assert!(game_state.can_place_queen(0, 0));
    assert!(game_state.can_place_queen(4, 3));
    assert!(game_state.can_place_queen(2, 7));
    assert!(game_state.can_place_queen(1, 5));
    assert!(game_state.can_place_queen(6, 7));
}

#[test]
fn test_is_goal_state() {
    let game_state = get_initial_game_state();

    let game_state = game_state.place_queen(0, 3);
    let game_state = game_state.place_queen(1, 5);
    let game_state = game_state.place_queen(2, 1);
    let game_state = game_state.place_queen(3, 7);
    let game_state = game_state.place_queen(4, 0);
    let game_state = game_state.place_queen(5, 2);
    let game_state = game_state.place_queen(6, 6);
    let game_state = game_state.place_queen(7, 4);

    assert!(game_state.is_goal_state());
}
