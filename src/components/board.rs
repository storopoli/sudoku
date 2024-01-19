//! # Board Module
//!
//! The `board` module is responsible for defining and rendering the Sudoku
//! board.
//! It includes the logic for laying out the cells in a grid,
//! handling user interactions,
//! and updating the state of the board as the game progresses.
//!
//! This module provides the [`SudokuBoard`] component,
//! which is the central element of the Sudoku game interface,
//! displaying the puzzle to the user and allowing interaction
//!  with individual cells.

use dioxus::prelude::*;

use crate::components::cell::{FreeCell, LockCell};
use crate::utils::create_sudoku;

/// Props for `SudokuBoard` component.
///
/// This struct contains the properties used by the [`SudokuBoard`] component.
/// It can optionally hold a predefined Sudoku puzzle, which is a
/// 9x9 grid represented as a flat array of 81 `u8` values.
/// Each value should be in the range 0-9, where 0 represents an empty cell.
///
/// # Examples
///
/// ```rust
/// let puzzle: [u8; 81] = [
///     // ... your Sudoku puzzle here ...
/// ];
/// let props = SudokuBoardProps { sudoku: Some(puzzle) };
/// ```
///
/// Or you can use a random generated Sudoku board:
///
/// ```rust
/// let props = SudokuBoardProps { sudoku: None };
/// ```
#[derive(Props, PartialEq)]
pub struct SudokuBoardProps {
    #[props(!optional)]
    sudoku: Option<[u8; 81]>,
}

/// Component to render a Sudoku board.
///
/// This component renders a Sudoku board which can be either randomly generated or
/// initialized with a predefined puzzle provided through [`SudokuBoardProps`].
///
/// Each cell in the board is represented as either a [`LockCell`]
/// or [`FreeCell`] depending on whether it is a part of the initial puzzle
/// or an empty cell that can be filled by the user.
///
/// # Example
///
/// ```rust
/// # use dioxus::prelude::*;
/// # fn main() {
/// let puzzle: [u8; 81] = [
///     // ... your Sudoku puzzle here ...
/// ];
/// cx.render(rsx!(SudokuBoard { sudoku: Some(puzzle) }))
/// # }
/// ```
///
/// Or you can use a random generated Sudoku board:
///
/// ```rust
/// # use dioxus::prelude::*;
/// # fn main() {
/// let puzzle: [u8; 81] = [
///     // ... your Sudoku puzzle here ...
/// ];
/// cx.render(rsx!(SudokuBoard { sudoku: None }))
/// # }
/// ```
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
