use crate::{CellState, GameState};
use owo_colors::OwoColorize;

use owo_colors::AnsiColors as oac;
const COLOR_TABLE: [owo_colors::AnsiColors; 14] = [
    oac::Red,
    oac::Green,
    oac::Yellow,
    oac::Blue,
    oac::Magenta,
    oac::Cyan,
    oac::BrightRed,
    oac::BrightGreen,
    oac::BrightYellow,
    oac::BrightBlue,
    oac::BrightMagenta,
    oac::BrightCyan,
    oac::White,
    oac::Black,
];

impl GameState {
    pub fn print_board(&self) {
        self.print_header();
        self.print_rows();
        println!();
    }

    fn print_header(&self) {
        let size = self.size;
        print!("   ");
        for i in 0..size {
            print!(" {} ", i.dimmed());
        }
        println!();
    }

    fn print_rows(&self) {
        let size = self.size;
        let states = self.states();
        let colors = self.colors();

        for r in 0..size {
            print!("{:2} ", r.dimmed());
            let row_offset = r * size;
            for c in 0..size {
                let idx = row_offset + c;
                self.print_cell(idx, states, colors);
            }
            println!();
        }
    }

    fn print_cell(&self, idx: usize, states: &[CellState], colors: &[u8]) {
        let color = COLOR_TABLE[(colors[idx] as usize) % COLOR_TABLE.len()];

        match states[idx] {
            CellState::Queen => {
                print!("{}", " ♛ ".on_color(color).black());
            }
            CellState::Blocked => {
                print!("{}", " ✖ ".on_color(color).black().dimmed());
            }
            CellState::Empty => {
                print!("{}", "   ".on_color(color));
            }
        }
    }
}
