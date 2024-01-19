//! # App Module
//!
//! The `app` module contains the main application logic for the Sudoku game.
//! It includes the setup and rendering of the main application component,
//! along with any global context or state required for the game.
//!
//! This module is responsible for initializing and launching the game,
//! setting up the environment, and managing the application lifecycle.

use dioxus::prelude::*;

use crate::components::board::SudokuBoard;

///
/// This function sets up the main environment for
/// the Sudoku game in a web browser, initializes the necessary state,
// and renders the main `SudokuBoard` component.
///
/// It is designed to be used as the root of the web application,
/// orchestrating the entire Sudoku game and its user interface.
pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(SudokuBoard { sudoku: None }))
}
