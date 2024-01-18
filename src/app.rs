use dioxus::prelude::*;

use crate::components::sudoku_board::SudokuBoard;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(SudokuBoard {}))
}
