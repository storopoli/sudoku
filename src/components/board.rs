use dioxus::prelude::*;

use crate::components::cell::Cell;
use crate::utils::create_sudoku;

pub fn SudokuBoard(cx: Scope) -> Element {
    let sudoku = create_sudoku();

    cx.render(rsx!(div {
        id: "container",
        for (index, &value) in sudoku.iter().enumerate() {
                Cell {
                    index: index as u8,
                    value: value,
                }
            }
    }))
}
