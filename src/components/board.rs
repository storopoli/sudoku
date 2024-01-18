use dioxus::prelude::*;

use crate::components::cell::{FreeCell, LockCell};
use crate::utils::create_sudoku;

pub fn SudokuBoard(cx: Scope) -> Element {
    let sudoku = create_sudoku();

    cx.render(rsx!(div {
        id: "container",
        for (index, &value) in sudoku.iter().enumerate() {
            if value == 0 {
                // Render FreeCell for empty cells
                rsx!(FreeCell {
                    index: index as u8,
                    value: value,
                })
            } else {
                // Render LockCell for non-empty cells
                rsx!(LockCell {
                    index: index as u8,
                    value: value,
                })
            }
            }
    }))
}
