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

use std::borrow::Cow;

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
#[derive(Props, Clone, PartialEq, Eq)]
pub struct CellProps {
    index: u8,
    value: u8,
    selected: bool,
    highlighted: bool,
    class: Cow<'static, str>,
    mutable: bool,
}

/// Represents a cell in a Sudoku puzzle.
///
/// This component allows for user interaction to select or enter a value.
///
/// # Props
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
/// # Errors
///
/// The component can panic if it cannot read the App's shared state on
/// the clicked cell and it's related cells.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::needless_pass_by_value)]
pub fn Cell(props: CellProps) -> Element {
    let value = props.value;
    let is_mutable = props.mutable;

    // Unpack all props and share states
    let id = props.index;
    let mut clicked = use_context::<Signal<Clicked>>();
    let mut mutable = use_context::<Signal<Mutable>>();
    let mut related = use_context::<Signal<Related>>();
    let conflicting = use_context::<Signal<Conflicting>>();

    // Conditionally display the value or an empty string
    let free = value != 0;
    let value = if free {
        value.to_string()
    } else {
        String::new()
    };
    // Conditionally have style
    let style = if clicked.read().0 == id {
        "background-color: #e4ebf2;".to_string()
    } else if conflicting.read().0.contains(&id) {
        "background-color: #d5656f;".to_string()
    } else if related.read().0.contains(&id) {
        "background-color: #c2ddf8;".to_string()
    } else {
        String::new()
    };

    rsx!(
        div {
            onclick: move |_| {
                clicked.write().0 = id;
                mutable.write().0 = is_mutable;
                related.write().0 = get_related_cells(id);
            },
            class: "{props.class}",
            id: "{id}",
            style: "{style}",
            "{&value}"
        }
    )
}
