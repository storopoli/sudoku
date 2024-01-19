use dioxus::prelude::*;

use crate::components::cell::{FreeCell, LockCell};
use crate::utils::create_sudoku;

#[derive(Props, PartialEq)]
pub struct SudokuBoardProps {
    #[props(!optional)]
    sudoku: Option<[u8; 81]>,
}

pub fn SudokuBoard(cx: Scope<SudokuBoardProps>) -> Element {
    let sudoku = cx.props.sudoku.unwrap_or(create_sudoku());

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

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_ssr::render;
    use regex::Regex;

    #[test]
    fn test_sudoku_board() {
        // Define the SudokuBoard component for testing
        let app: Component = |cx| cx.render(rsx!(SudokuBoard { sudoku: None }));

        // Create a virtual DOM instance with the component
        let mut vdom = VirtualDom::new(app);

        // Rebuild the virtual DOM to ensure it's up-to-date
        let _ = vdom.rebuild();

        // Render the virtual DOM to a string
        let rendered_sudoku_board = render(&vdom);

        // Regex to find div elements with id and class
        let re = Regex::new(r#"<div id="(\d+)" class="([^"]+)""#).unwrap();

        for cap in re.captures_iter(&rendered_sudoku_board) {
            let id: i32 = cap[1].parse().unwrap();

            // Check if all div IDs are between 0 and 80
            assert!(id >= 0 && id < 81);
        }
    }
}
