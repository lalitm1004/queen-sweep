use queen_sweep_core::{CellState, GameState};

const RESET: &str = "\x1b[0m";
const BLACK_FG: &str = "\x1b[30m";
const DIM: &str = "\x1b[2m";

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
    print_header(game_state.size());
    print_board(game_state);
    println!();
}

fn print_header(size: usize) {
    print!("   ");
    for i in 0..size {
        print!("{} ", i);
    }
    println!();
}

fn print_board(game_state: &GameState) {
    for r in 0..game_state.size() {
        print!("{:2} ", r);

        for c in 0..game_state.size() {
            let idx = r * game_state.size() + c;
            print_cell(game_state, idx);
        }

        println!();
    }
}

fn print_cell(game_state: &GameState, idx: usize) {
    let color_index = (game_state.colors()[idx] as usize) % BACK_COLORS.len();
    let back_color = BACK_COLORS[color_index];

    match game_state.states()[idx] {
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
