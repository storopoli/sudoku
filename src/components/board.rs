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

use crate::app::SudokuState;
use crate::components::cell::Cell;
use crate::utils::{create_sudoku, get_class, get_conflicting_cells};

/// Shared State for clicked [`Cell`]
///
/// Represents globally across the app which cell is clicked by id.
pub struct Clicked(pub u8);

/// Shared State for mutable clicked [`Cell`]
///
/// Represents globally across the app if the clicke cell is mutable.
/// Imutable cells are the one created by the app at the initial puzzle creation.
pub struct Mutable(pub bool);

/// Shared State for clicked [`Cell`]'s related [`Cell`]s
///
/// Represents globally across the app which cells, by id,
/// are related to the clicked [`Cell`].
///
/// [`Cell`]s are related if they share the same row, column, or sub-grid in
/// a Sudoku board.
///
/// See also: [`get_related_cells`](crate::utils::get_related_cells).
pub struct Related(pub Vec<u8>);

/// Shared State for clicked [`Cell`]'s conficts
///
/// Represents globally across the app which cells, by id,
/// are in conflict to the clicked [`Cell`].
///
/// [`Cell`]s are in conflict if they share the same row, column, or sub-grid in
/// a Sudoku board and have the same value.
///
/// See also: [`get_related_cells`](crate::utils::get_related_cells)
/// and [`get_conflicting_cells`].
pub struct Conflicting(pub Vec<u8>);

/// Shared State for the initial [`SudokuBoard`] puzzle
pub struct InitialSudokuPuzzle(pub SudokuState);
impl InitialSudokuPuzzle {
    #[must_use]
    pub fn new() -> Self {
        Self(create_sudoku())
    }
}
impl Default for InitialSudokuPuzzle {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared State for the current [`SudokuBoard`] puzzle
pub struct SudokuPuzzle(pub SudokuState);

/// Component Props for [`NumberButton`]
///
/// - `number: u8`: the value to be rendered in the button and also the value
///   that dictates what the button will write to a mutable cell.
#[derive(Props, PartialEq, Eq)]
struct NumberButtonProps {
    number: u8,
}

/// Component to render a number button
///
/// This component renders buttons that are used to interact with the board.
/// They work by assigning value to a mutable cell.
///
/// The number 0 is a special button that renders as a delete icon.
/// It adds the number 0 to a mutable cell which represents an empty cell.
fn NumberButton(cx: Scope<NumberButtonProps>) -> Element {
    let number = cx.props.number;
    let class: &str = match number {
        0 => "input icon delete",
        _ => "input number",
    };

    // Unpack shared states
    let sudoku =
        use_shared_state::<SudokuPuzzle>(cx).expect("failed to get sudoku puzzle shared state");
    let conflicting =
        use_shared_state::<Conflicting>(cx).expect("failed to get conflicting cells shared state");
    let clicked = use_shared_state::<Clicked>(cx)
        .expect("failed to get clicked cell shared state")
        .read()
        .0;
    let mutable = use_shared_state::<Mutable>(cx)
        .expect("failed to get clicked cell mutability shared state")
        .read()
        .0;

    cx.render(rsx!(
        button {
            class: "{class}",
            onclick: move |_| {
                // if the cell is mutable
                if mutable {
                    // chaging the clicked cell value to the button number
                    sudoku.write().0[clicked as usize] = number;
                    let current_sudoku = sudoku.read().0;

                    // conflicting logic
                    let filled: Vec<u8> = sudoku
                        .read()
                        .0
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, &value)| {
                            if value != 0 {
                                if let Some(index) = u8::try_from(idx).ok() {
                                    // Safely convert usize to u8 if within range
                                    Some(index) // Return Some(index) if conversion succeeds
                                } else {
                                    // Handle the case where usize does not fit in u8
                                    None // or panic, or log an error, etc.
                                }
                            } else {
                                None // Filter out the item if the value is 0
                            }
                        })
                        .collect();
                    let mut new_conflicting: Vec<u8> = filled.iter().flat_map(|&v| get_conflicting_cells(&current_sudoku, v))
                    .collect::<Vec<u8>>();
                    new_conflicting.sort_unstable();
                    new_conflicting.dedup();
                    conflicting.write().0 = new_conflicting;
                }
            },
            "{number}"
        }
    ))
}

/// Component to render a new button
///
/// This component renders a "New Game" button.
/// When activated, all current state is dropped and the board is drawn with a
/// fresh new puzzle for the user.
fn NewButton(cx: Scope) -> Element {
    // Unpack shared states
    let initial_sudoku = use_shared_state::<InitialSudokuPuzzle>(cx)
        .expect("failed to get initial sudoku puzzle shared state");
    let sudoku =
        use_shared_state::<SudokuPuzzle>(cx).expect("failed to get sudoku puzzle shared state");
    let clicked = use_shared_state::<Clicked>(cx).expect("failed to get clicked cell shared state");
    let mutable = use_shared_state::<Mutable>(cx)
        .expect("failed to get clicked cell mutability shared state");
    let related =
        use_shared_state::<Related>(cx).expect("failed to get related cells shared state");
    let conflicting =
        use_shared_state::<Conflicting>(cx).expect("failed to get conflicting cells shared state");

    cx.render(rsx!(button {
        class: "input icon new",
        onclick: move |_| {
            // resetting the board with a new puzzle
            initial_sudoku.write().0 = create_sudoku();
            sudoku.write().0 = initial_sudoku.read().0;
            // resetting the clicked cell
            clicked.write().0 = 90;
            // resetting the mutable cell
            mutable.write().0 = true;
            // resetting the related list
            related.write().0 = vec![];
            // resetting the conflicting list
            conflicting.write().0 = vec![];
        }
    }))
}

/// Component to render a Sudoku board.
///
/// This component renders a Sudoku board which can be either randomly generated.
///
/// Each cell in the board is represented as a [`Cell`].
/// Cells that were generated by the initial puzzle are not mutable,
/// i.e cannot have the values changed by the user.
///
/// ## Panics
///
/// The component will panic if cannot convert any of the Sudoku's board cells
/// indexes from `usize` into a `u8`
///
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn SudokuBoard(cx: Scope) -> Element {
    // Initialize all shared states
    use_shared_state_provider(cx, || Clicked(90)); // this will never imply in a highlighted cell at initial state
    use_shared_state_provider(cx, || Mutable(false));
    use_shared_state_provider(cx, || Related(vec![]));
    use_shared_state_provider(cx, || Conflicting(vec![]));

    // Unpack shared states
    let initial_sudoku = use_shared_state::<InitialSudokuPuzzle>(cx)
        .expect("failed to get initial sudoku puzzle shared state")
        .read()
        .0;
    let sudoku = use_shared_state::<SudokuPuzzle>(cx)
        .expect("failed to get sudoku puzzle shared state")
        .read()
        .0;
    let clicked = use_shared_state::<Clicked>(cx);

    cx.render(rsx!(div {
        id: "container",

        // Render Cells
        for (index, &value) in sudoku.iter().enumerate() {
                rsx!(Cell {
                    index: u8::try_from(index).expect("cannot convert from u8"),
                    value: value,
                    selected: clicked.expect("failed to get clicked shared state").read().0 == u8::try_from(index).expect("cannot convert from u8"),
                    highlighted: false,
                    class: get_class(u8::try_from(index).expect("cannot convert from u8"), initial_sudoku[index] == 0),
                    mutable: initial_sudoku[index] == 0,
                })
            }

        // Render NumberButtons
        for i in 1..=9 {
            NumberButton {
                number: i
            }
        }

        // Render "DeleteButton", a.k.a number is 0
        NumberButton {
            number: 0,
        }
        // Render NewButton
        NewButton{}
    }))
}
