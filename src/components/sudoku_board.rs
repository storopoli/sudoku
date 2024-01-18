use dioxus::prelude::*;
use sudoku::board::Sudoku;

pub fn SudokuBoard(cx: Scope) -> Element {
    let sudoku = Sudoku::generate();
    let line = sudoku.to_str_line();
    cx.render(rsx!(
        div {"{line}"}
    ))
}
