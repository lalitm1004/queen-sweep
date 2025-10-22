use queen_sweep_core::{CellState, GameState, depth_first_search};

fn main() {
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
    let state = GameState::from_color_regions(color_regions);
    pretty_print(&state);

    let solved = depth_first_search(state);
    if let Some(solved) = solved {
        pretty_print(&solved);
    } else {
        println!("No Solution");
    }
}

// ANSI color codes
const RESET: &str = "\x1b[0m";
const BLACK_FG: &str = "\x1b[30m";
const DIM: &str = "\x1b[2m";

// Background colors matching Python's colorama
const BACK_COLORS: [&str; 14] = [
    "\x1b[101m", // LIGHTRED_EX
    "\x1b[102m", // LIGHTGREEN_EX
    "\x1b[103m", // LIGHTYELLOW_EX
    "\x1b[104m", // LIGHTBLUE_EX
    "\x1b[105m", // LIGHTMAGENTA_EX
    "\x1b[106m", // LIGHTCYAN_EX
    "\x1b[41m",  // RED
    "\x1b[42m",  // GREEN
    "\x1b[43m",  // YELLOW
    "\x1b[44m",  // BLUE
    "\x1b[45m",  // MAGENTA
    "\x1b[46m",  // CYAN
    "\x1b[47m",  // WHITE
    "\x1b[40m",  // BLACK
];

pub fn pretty_print(game_state: &GameState) {
    // Print column headers
    print!("   ");
    for i in 0..game_state.size {
        print!("{} ", i);
    }
    println!();

    // Print each row
    for r in 0..game_state.size {
        print!("{:2} ", r);

        for c in 0..game_state.size {
            let idx = r * game_state.size + c;
            let color_index = (game_state.colors[idx] as usize) % BACK_COLORS.len();
            let back_color = BACK_COLORS[color_index];

            match game_state.states[idx] {
                CellState::Queen => {
                    print!("{}{}♛ {}", back_color, BLACK_FG, RESET);
                }
                CellState::Blocked => {
                    print!("{}{}{}✖ {}", back_color, BLACK_FG, DIM, RESET);
                }
                CellState::Empty => {
                    print!("{}  {}", back_color, RESET);
                }
            }
        }

        println!();
    }

    println!();
}
