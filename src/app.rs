use dioxus::prelude::*;

use crate::components::board::SudokuBoard;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(SudokuBoard {}))
}
