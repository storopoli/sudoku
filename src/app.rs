//! # App Module
//!
//! The `app` module contains the main application logic for the Sudoku game.
//! It includes the setup and rendering of the main application component,
//! along with any global context or state required for the game.
//!
//! This module is responsible for initializing and launching the game,
//! setting up the environment, and managing the application lifecycle.

use dioxus::prelude::*;

use crate::components::board::{InitialSudokuPuzzle, SudokuBoard, SudokuPuzzle, SudokuPuzzleMoves};

/// Represents a Sudoku state with the values, as `u8`, of the 81 cells in a
/// Sodoku game
pub type SudokuState = [u8; 81];

/// This function sets up the main environment for
/// the Sudoku game in a web browser, initializes the necessary state,
/// and renders the main [`SudokuBoard`] component.
///
/// It is designed to be used as the root of the web application,
/// orchestrating the entire Sudoku game and its user interface.
///
/// ## Panics
///
/// The app will panic if fails to get initial Sudoku puzzle shared state.
#[component]
pub fn App() -> Element {
    // set initial puzzle
    use_context_provider(|| Signal::new(InitialSudokuPuzzle::new()));

    // set current sudoku and cache of user moves
    let initial_sudoku = use_context::<Signal<InitialSudokuPuzzle>>()
        .read()
        .0;
    use_context_provider(|| Signal::new(SudokuPuzzle(initial_sudoku)));
    use_context_provider(|| Signal::new(SudokuPuzzleMoves(vec![initial_sudoku])));

    rsx!(
        h1 {
            class: "input",
            "Sudoku"
        }

        SudokuBoard {}

        div{
            class: "github",
            a {
                class: "github input",
                href: "https://github.com/storopoli/sudoku",
                target: "_blank",
                rel: "noopener noreferrer",
                svg {
                    class: "github",
                    view_box: "0 0 24 24",
                    fill: "#3b59a9",
                    path {
                        d: "M12 2C6.477 2 2 6.477 2 12c0 4.418 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.009-.866-.014-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.003.07 1.532 1.03 1.532 1.03.891 1.529 2.341 1.088 2.912.833.091-.646.349-1.086.635-1.337-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.03-2.682-.103-.253-.447-1.27.098-2.646 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 7.07c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.026 2.747-1.026.547 1.376.203 2.394.1 2.646.64.699 1.028 1.591 1.028 2.682 0 3.841-2.337 4.687-4.565 4.934.359.31.678.92.678 1.852 0 1.336-.012 2.415-.012 2.741 0 .267.18.578.688.48A10.017 10.017 0 0022 12C22 6.477 17.523 2 12 2z"
                    }
                }
                "storopoli/sudoku"
            }
        }
    )
}
