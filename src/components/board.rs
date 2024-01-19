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
    use dioxus_ssr::render_lazy;
    use rand::Rng;
    use regex::Regex;

    fn create_random_full_sudoku() -> [u8; 81] {
        let mut rng = rand::thread_rng();
        let mut board = [0u8; 81];
        for item in &mut board {
            *item = rng.gen_range(1..=9);
        }
        board
    }

    fn generate_100_full_sudoku_boards() -> Vec<[u8; 81]> {
        (0..100).map(|_| create_random_full_sudoku()).collect()
    }

    #[test]
    fn test_sudoku_board() {
        // Render the Component into a string
        let rendered_sudoku_board = render_lazy(rsx!(SudokuBoard { sudoku: None }));

        // Regex to find div elements with id and class
        let re = Regex::new(r#"<div id="(\d+)" class="([^"]+)""#).unwrap();

        for cap in re.captures_iter(&rendered_sudoku_board) {
            let id: i32 = cap[1].parse().unwrap();

            // Check if all div IDs are between 0 and 80
            assert!((0..81).contains(&id));
        }
    }

    #[test]
    fn test_100_full_sudoku_boards() {
        let full_boards = generate_100_full_sudoku_boards();
        let re = Regex::new(r#"<div[^>]*>([^<]*)</div>"#).unwrap();

        for board in full_boards {
            let rendered_board = render_lazy(rsx!(SudokuBoard {
                sudoku: Some(board)
            }));

            for cap in re.captures_iter(&rendered_board) {
                let inner_text = &cap[1];
                assert!(
                    !inner_text.trim().is_empty(),
                    "Found empty cell in board: {}",
                    rendered_board
                );
            }
        }
    }
}
