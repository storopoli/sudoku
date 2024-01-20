//! # Cell Module
//!
//! The `cell` module contains components representing individual cells in a
//! Sudoku puzzle.
//! It includes a single component, [`Cell`],
//! that represents a cell in a  Sudoku board.
//!
//! [`Cell`] is integral to the Sudoku puzzle,
//! providing interactive elements that players use to input their answers.
//! Each cell component is responsible for rendering its content
//! and handling user input,
//! while conforming to the overall style and rules of the Sudoku game.

use crate::components::board::Clicked;
use crate::utils::get_related_cells;
use dioxus::prelude::*;

use super::board::{Conflicting, Mutable, Related};

/// Component Props for [`Cell`]
///
/// - `index: u8`: The unique identifier of the cell in the grid,
///   ranging from 0 to 80.
///   The index is used to calculate the cell's position in the grid
///   and to derive its CSS class.
/// - `value: u8`: The current value of the cell.
/// - `selected: bool`: If the cell is current clicked by the user.
///   Cells that are clicked, should be highlighted.
/// - `highlighted: bool`: If the cell is related to the clicked cells.
///   Cells are related if the share the same row, column, or sub-grid
///   in a Sudoku board.
/// - `class: &str`: The cell's CSS class.
///   Handled by the [`get_class`](crate::utils::get_class) function.
/// - `mutable: bool`: If the cell's value can be changed by the user.
///   Mutable cells are the ones that are blank when the Sudoku board is
///   generated.
#[allow(clippy::module_name_repetitions)]
#[derive(Props, PartialEq, Eq)]
pub struct CellProps<'a> {
    index: u8,
    value: u8,
    selected: bool,
    highlighted: bool,
    class: &'a str,
    mutable: bool,
}

/// Represents a cell in a Sudoku puzzle.
///
/// This component allows for user interaction to select or enter a value.
///
/// ## Props
///
/// - `index: u8`: The unique identifier of the cell in the grid,
///   ranging from 0 to 80.
///   The index is used to calculate the cell's position in the grid
///   and to derive its CSS class.
/// - `value: u8`: The current value of the cell.
/// - `selected: bool`: If the cell is current clicked by the user.
///   Cells that are clicked, should be highlighted.
/// - `highlighted: bool`: If the cell is related to the clicked cells.
///   Cells are related if the share the same row, column, or sub-grid
///   in a Sudoku board.
/// - `class: &str`: The cell's CSS class.
///   Handled by the [`get_class`](crate::utils::get_class) function.
/// - `mutable: bool`: If the cell's value can be changed by the user.
///   Mutable cells are the ones that are blank when the Sudoku board is
///   generated.
///
/// ## Panics
///
/// The component can panic if it cannot read the App's shared state on
/// the clicked cell and it's related cells.
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn Cell<'a>(cx: Scope<'a, CellProps<'a>>) -> Element<'a> {
    let value = cx.props.value;
    let is_mutable = cx.props.mutable;

    // Unpack all props and share states
    let id = cx.props.index;
    let clicked = use_shared_state::<Clicked>(cx).expect("failed to get clicked cell shared state");
    let mutable = use_shared_state::<Mutable>(cx)
        .expect("failed to get clicked cell mutability shared state");
    let related =
        use_shared_state::<Related>(cx).expect("failed to get related cells shared state");
    let conflicting =
        use_shared_state::<Conflicting>(cx).expect("failed to get conflicting cells shared state");

    // Conditionally display the value or an empty string
    let free = value != 0;
    let mut value = value.to_string();
    if !free {
        value = String::new();
    };

    // Conditionally have style
    let mut style = String::new();
    if related.read().0.contains(&id) {
        style = "background-color: #c2ddf8;".to_string();
    };
    if conflicting.read().0.contains(&id) {
        style = "background-color: #d5656f;".to_string();
    };
    if clicked.read().0 == id {
        style = "background-color: #e4ebf2;".to_string();
    }

    cx.render(rsx!(
        div {
            onclick: move |_| {
                clicked.write().0 = id;
                mutable.write().0 = is_mutable;
                related.write().0 = get_related_cells(id);
            },
            class: "{cx.props.class}",
            id: "{id}",
            style: "{style}",
            "{&value}"
        }
    ))
}
