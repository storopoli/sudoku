use dioxus::prelude::*;

use crate::utils::create_sudoku;

fn get_class(id: usize) -> &'static str {
    match id {
        0 | 3 | 6 | 27 | 30 | 33 | 54 | 57 | 60 => "tsb lsb rdb bdb",
        1 | 4 | 7 | 28 | 31 | 34 | 55 | 58 | 61 => "tsb  bdb",
        2 | 5 | 29 | 32 | 56 | 59 => "tsb bdb ldb",
        8 | 35 | 62 => "tsb rsb bdb ldb",
        9 | 12 | 15 | 36 | 39 | 42 | 63 | 66 | 69 => "lsb rdb bdb",
        10 | 13 | 16 | 37 | 40 | 43 | 64 | 67 | 70 => "bdb",
        11 | 14 | 38 | 41 | 65 | 68 => "bdb ldb",
        18 | 21 | 24 | 45 | 48 | 51 => "lsb rdb",
        20 | 23 | 47 | 50 => "ldb",
        17 | 44 | 71 => "rsb ldb bdb",
        26 | 53 => "rsb ldb",
        72 | 75 | 78 => "bsb lsb rdb",
        73 | 76 | 79 => "bsb",
        74 | 77 => "bsb ldb",
        80 => "bsb rsb ldb",
        _ => "",
    }
}

pub fn SudokuBoard(cx: Scope) -> Element {
    let sudoku = create_sudoku();

    cx.render(rsx!(
        div {
            id: "container",
            for (index, &value) in sudoku.iter().enumerate() {
                div {
                    class: "{get_class(index)}",
                    id: "{index}",
                    if value != 0 {
                        rsx!("{value}")
                    } else {
                        rsx!("")
                    }
                }
            }
        }
    ))
}
