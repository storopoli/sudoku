//! # Sudoku App
//!
//! This crate is a Rust binary for generating and solving Sudoku puzzles.
//! It provides functionality to create random Sudoku puzzles, solve them,
//! and manipulate or interact with Sudoku boards in various ways.
//!
//! ## Features
//!
//! - Generate random Sudoku puzzles.
//! - Solve Sudoku puzzles programmatically.
//! - Render Sudoku puzzles as interactive boards in a web interface using Dioxus.
//! - Support for various difficulty levels and puzzle variants.
//!
//! ## Usage
//!
//! To render a Sudoku board in a web interface:
//!
//! ```rust
//! use dioxus::prelude::*;
//!
//! use crate::components::board::SudokuBoard;
//!
//! pub fn App(cx: Scope) -> Element {
//!     cx.render(rsx!(SudokuBoard { sudoku: None }))
//! }
//! ```
//!
//! ## Dependecies
//!
//! Under the hood Sudoku app uses:
//!
//! - [Dioxus](https://dioxuslabs.com/):  fullstack, crossplatform, fast Apps.
//! - [`sudoku`](https::/docs.rs/sudoku): sudoku solving and generator.
//!
//! ## Contributing
//!
//! Contributions to the are welcome!
//! Whether it's bug fixes, feature implementations,
//! or improvements to the documentation,
//! feel free to create pull requests or open issues.
//!
//! ## License
//!
//! This crate is licensed under
//! [MIT license](https://opensource.org/licenses/MIT).

#![allow(non_snake_case)]
use dioxus_web::launch;

#[cfg(debug_assertions)]
use log::{info, LevelFilter};

pub mod app;
pub mod components;
pub mod utils;

use app::App;

/// Main entry point for the Sudoku web application.
///
/// This function initializes and launches the Sudoku game in a web browser
/// using the Dioxus framework.
/// It sets up the necessary environment and  renders the main `SudokuBoard`
/// component, along with any additional components or context providers
/// required for the application.
///
/// ## Panics
///
/// If in debug mode, this function will panic if it fails to initialize the
/// Dioxuslogger.
pub fn main() {
    #[cfg(debug_assertions)]
    {
        // init logger for Dioxus
        dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    }
    // launch the web app
    #[cfg(debug_assertions)]
    info!("Launching Sudoku app");
    launch(App);
}
