//! # Board Module
//!
//! The `board` module is responsible for defining and rendering the Sudoku
//! board.
//! It includes the logic for laying out the cells in a grid,
//! handling user interactions,
//! and updating the state of the board as the game progresses.
//!
//! This module provides the [`SudokuBoard()`] component,
//! which is the central element of the Sudoku game interface,
//! displaying the puzzle to the user and allowing interaction
//!  with individual cells.

use dioxus::prelude::*;

use crate::app::SudokuState;
use crate::components::cell::Cell;
use crate::utils::{
    create_sudoku, find_changed_cell, get_all_conflicting_cells, get_class, get_hint,
    get_related_cells, remove_conflicting_cells,
};

/// Shared State for clicked [`Cell`]
///
/// Represents globally across the app which cell is clicked by id.
#[derive(Debug, Clone)]
pub struct Clicked(pub u8);

/// Shared State for mutable clicked [`Cell`]
///
/// Represents globally across the app if the clicke cell is mutable.
/// Imutable cells are the one created by the app at the initial puzzle creation.
#[derive(Debug, Clone)]
pub struct Mutable(pub bool);

/// Shared State for clicked [`Cell`]'s related [`Cell`]s
///
/// Represents globally across the app which cells, by id,
/// are related to the clicked [`Cell`].
///
/// [`Cell`]s are related if they share the same row, column, or sub-grid in
/// a Sudoku board.
///
/// See also: [`get_related_cells`].
#[derive(Debug, Clone)]
pub struct Related(pub Vec<u8>);

/// Shared State for clicked [`Cell`]'s conficts
///
/// Represents globally across the app which cells, by id,
/// are in conflict to the clicked [`Cell`].
///
/// [`Cell`]s are in conflict if they share the same row, column, or sub-grid in
/// a Sudoku board and have the same value.
///
/// See also: [`get_related_cells`]
/// and [`get_conflicting_cells`](crate::utils::get_conflicting_cells).
#[derive(Debug, Clone)]
pub struct Conflicting(pub Vec<u8>);

/// Shared State for the initial [`SudokuBoard()`] puzzle
#[derive(Debug, Clone)]
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

/// Shared State for the current [`SudokuBoard()`] puzzle
#[derive(Debug, Clone)]
pub struct SudokuPuzzle(pub SudokuState);

/// Shared State for the all the [`SudokuState`] across user moves
#[derive(Debug, Clone)]
pub struct SudokuPuzzleMoves(pub Vec<SudokuState>);

/// Component Props for [`NumberButton`]
///
/// - `number: u8`: the value to be rendered in the button and also the value
///   that dictates what the button will write to a mutable cell.
#[derive(Props, Copy, Clone, PartialEq, Eq)]
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
#[component]
fn NumberButton(props: NumberButtonProps) -> Element {
    let number = props.number;
    let class: &str = match number {
        0 => "input icon delete",
        _ => "input number",
    };

    // Unpack shared states
    let mut moves = use_context::<Signal<SudokuPuzzleMoves>>();
    let mut sudoku = use_context::<Signal<SudokuPuzzle>>();
    let mut conflicting = use_context::<Signal<Conflicting>>();
    let clicked = use_context::<Signal<Clicked>>().read().0;
    let mutable = use_context::<Signal<Mutable>>().read().0;

    rsx!(
        button {
            class: "{class}",
            onclick: move |_| {
                // if the value is the same
                if sudoku.read().0[clicked as usize] == number {
                }
                // if the cell is mutable
                 else if mutable {
                    // chaging the clicked cell value to the button number
                    sudoku.write().0[clicked as usize] = number;
                    let current_sudoku = sudoku.read().0;
                    moves.write().0.push(current_sudoku);

                    // conflicting logic
                    let new_conflicting = get_all_conflicting_cells(&current_sudoku);
                    conflicting.write().0 = new_conflicting;
                }
            },
            "{number}"
        }
    )
}

/// Component to render a new button
///
/// This component renders a "New Game" button.
/// When activated, all current state is dropped and the board is drawn with a
/// fresh new puzzle for the user.
#[component]
fn NewButton() -> Element {
    // Unpack shared states
    let mut initial_sudoku = use_context::<Signal<InitialSudokuPuzzle>>();
    let mut moves = use_context::<Signal<SudokuPuzzleMoves>>();
    let mut sudoku = use_context::<Signal<SudokuPuzzle>>();
    let mut clicked = use_context::<Signal<Clicked>>();
    let mut mutable = use_context::<Signal<Mutable>>();
    let mut related = use_context::<Signal<Related>>();
    let mut conflicting = use_context::<Signal<Conflicting>>();

    rsx!(button {
        class: "input icon new",
        onclick: move |_| {
            // resetting the board with a new puzzle
            initial_sudoku.write().0 = create_sudoku();
            moves.write().0 = vec![initial_sudoku.read().0];
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
    })
}

/// Component to render an undo button
///
/// This component renders a "Undo" button.
/// When activated, all current state is dropped and the board is drawn with a
/// fresh new puzzle for the user.
#[component]
fn UndoButton() -> Element {
    // Unpack shared states
    let initial_sudoku = use_context::<Signal<InitialSudokuPuzzle>>().read().0;
    let mut moves = use_context::<Signal<SudokuPuzzleMoves>>();
    let current_sudoku = *moves
        .read()
        .0
        .last()
        .expect("failed to get the current sudoku state");
    let mut sudoku = use_context::<Signal<SudokuPuzzle>>();
    let mut clicked = use_context::<Signal<Clicked>>();
    let mut related = use_context::<Signal<Related>>();
    let mut conflicting = use_context::<Signal<Conflicting>>();

    rsx!(button {
        class: "input icon undo",
        onclick: move |_| {
            if current_sudoku == initial_sudoku {
                let new_conflicting = conflicting.read().0.clone();
                conflicting.write().0 = new_conflicting;
            } else {
                // pop the last element of moves
                let last_state = moves
                    .write()
                    .0
                    .pop()
                    .expect("cannot pop the last element of the sudoku moves shared state");

                let new_sudoku = *moves
                    .read()
                    .0
                    .last()
                    .expect("failed to get sudoku moves shared state");
                // resetting the board with a new puzzle
                sudoku.write().0 = new_sudoku;

                // update clicked, related
                let last_clicked = find_changed_cell(&last_state, &new_sudoku)
                    .expect("cannot find changed index between the two previous state");
                clicked.write().0 = last_clicked;
                related.write().0 = get_related_cells(last_clicked);

                // conflicting logic
                let new_conflicting = get_all_conflicting_cells(&new_sudoku);
                conflicting.write().0 = new_conflicting;
            }
        }
    })
}

/// Component to render a hint button
///
/// This component renders a "Hint" button.
/// When activated, the button will give the user a hint by filling a cell with a value.
/// It also handles the UI updates for the clicked, related and conflicting cells.
///
/// # Panics
///
/// The component will panic if cannot find the changed cell between the two previous states.
#[component]
pub fn HintButton() -> Element {
    // Unpack shared states
    let mut moves = use_context::<Signal<SudokuPuzzleMoves>>();
    let current_sudoku = *moves
        .read()
        .0
        .last()
        .expect("failed to get the current sudoku state");
    let mut sudoku = use_context::<Signal<SudokuPuzzle>>();
    let mut clicked = use_context::<Signal<Clicked>>();
    let mut related = use_context::<Signal<Related>>();
    let mut conflicting = use_context::<Signal<Conflicting>>();

    rsx!(button {
        class: "input icon hint",
        onclick: move |_| {
            #[cfg(debug_assertions)]
            log::info!("entering hint button onclick event handler");

            // If there are conflicting cells, remove all of them
            if !conflicting.read().0.is_empty() {
                #[cfg(debug_assertions)]
                log::info!("conflicting cells found, removing them");

                let mut current_sudoku = current_sudoku;
                let conficting_cells = get_all_conflicting_cells(&current_sudoku);
                remove_conflicting_cells(&mut current_sudoku, &conficting_cells);

                // update the moves state with new sudoku
                moves.write().0.push(current_sudoku);

                // update the conflicting state
                conflicting.write().0 = vec![];

                // update the sudoku state
                sudoku.write().0 = current_sudoku;
            }

            // Try to get a hint
            let new_sudoku = get_hint(&sudoku.read().0).unwrap_or_else(|_| {
                // If no hint found, try again after removing conflicts
                let mut current_sudoku = sudoku.read().0;
                let conficting_cells = get_all_conflicting_cells(&current_sudoku);
                remove_conflicting_cells(&mut current_sudoku, &conficting_cells);

                get_hint(&current_sudoku).expect("no hint found even after removing conflicts")
            });

            // If the sudoku is not complete, then update the states
            if sudoku.read().0.iter().any(|&val| val == 0) {
                #[cfg(debug_assertions)]
                log::info!("sudoku is not complete, updating states");

                // get the last clicked cell
                let last_clicked = find_changed_cell(&sudoku.read().0, &new_sudoku)
                    .expect("cannot find changed index between the two previous state");

                // update all states
                sudoku.write().0 = new_sudoku;

                moves.write().0.push(new_sudoku);
                clicked.write().0 = last_clicked;
                related.write().0 = get_related_cells(last_clicked);
                conflicting.write().0 = get_all_conflicting_cells(&new_sudoku);
            }
        }
    })
}

/// Component to render a Sudoku board.
///
/// This component renders a Sudoku board which can be either randomly generated.
///
/// Each cell in the board is represented as a [`Cell`].
/// Cells that were generated by the initial puzzle are not mutable,
/// i.e cannot have the values changed by the user.
///
/// # Panics
///
/// The component will panic if cannot convert any of the Sudoku's board cells
/// indexes from `usize` into a `u8`
///
#[allow(clippy::module_name_repetitions)]
#[component]
pub fn SudokuBoard() -> Element {
    // Initialize all shared states
    use_context_provider(|| Signal::new(Clicked(90))); // this will never imply in a highlighted cell at initial state
    use_context_provider(|| Signal::new(Mutable(false)));
    use_context_provider(|| Signal::new(Related(vec![])));
    use_context_provider(|| Signal::new(Conflicting(vec![])));

    // Unpack shared states
    let initial_sudoku = use_context::<Signal<InitialSudokuPuzzle>>().read().0;
    let moves = use_context::<Signal<SudokuPuzzleMoves>>();

    let sudoku = &moves.read().0;
    let last_sudoku = sudoku
        .last()
        .expect("failed to get the last element of the sudoku moves shared state");

    let clicked = use_context::<Signal<Clicked>>();

    rsx!(div {
        id: "container",

        // Render Cells
        for (index, &value) in last_sudoku.iter().enumerate() {
            Cell {
                index: u8::try_from(index).expect("cannot convert from u8"),
                value: value,
                    selected: clicked.read().0 == u8::try_from(index).expect("cannot convert from u8"),
                    highlighted: false,
                    class: get_class(u8::try_from(index).expect("cannot convert from u8"), initial_sudoku[index] == 0),
                    mutable: initial_sudoku[index] == 0,
                }
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

        // Render HintButton
        HintButton{}

        // Render UndoButton
        UndoButton{}

        // Render NewButton
        NewButton{}
    })
}
