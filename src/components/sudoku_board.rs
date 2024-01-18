use dioxus::prelude::*;
use sudoku::board::Sudoku;

fn get_class(id: u8) -> &'static str {
    match id {
        1 | 4 | 7 | 28 | 31 | 34 | 55 | 58 | 61 => "tsb lsb rdb bdb",
        2 | 5 | 8 | 29 | 32 | 35 | 56 | 59 | 62 => "tsb  bdb",
        3 | 6 | 30 | 33 | 57 | 60 => "tsb bdb ldb",
        9 | 36 | 63 => "tsb rsb bdb ldb",
        10 | 13 | 16 | 37 | 40 | 43 | 64 | 67 | 70 => "lsb rdb bdb",
        11 | 14 | 17 | 38 | 41 | 44 | 65 | 68 | 71 => "bdb",
        12 | 15 | 39 | 42 | 66 | 69 => "bdb ldb",
        19 | 22 | 25 | 46 | 49 | 52 => "lsb rdb",
        21 | 24 | 48 | 51 => "ldb",
        18 | 45 | 72 => "rsb ldb bdb",
        27 | 54 => "rsb ldb",
        73 | 76 | 79 => "bsb lsb rdb",
        74 | 77 | 80 => "bsb",
        75 | 78 => "bsb ldb",
        81 => "bsb rsb ldb",
        _ => "",
    }
}

pub fn SudokuBoard(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            id: "container",
            (1..=81).map(|id| {
                rsx! {
                    div { class: "{get_class(id)}", id: "{id}" }
                }
            })
        }
    ))
}
