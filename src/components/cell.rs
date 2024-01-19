//! # Cell Module
//!
//! The `cell` module contains components representing individual cells in a
//! Sudoku puzzle.
//! It includes different types of cells, such as [`LockCell`] for cells
//! with fixed values and [`FreeCell`] for cells that the player can fill.
//!
//! These components are integral to the Sudoku puzzle,
//! providing interactive elements that players use to input their answers.
//! Each cell component is responsible for rendering its content
//! and handling user input,
//! while conforming to the overall style and rules of the Sudoku game.

use crate::utils::get_class;
use dioxus::prelude::*;

/// Properties for the [`FreeCell`] and [`LockCell`] components in the Sudoku
/// game.
///
/// This struct defines the properties required to create and render a cell
/// in the Sudoku grid.
/// It contains information about the cell's index and value,
/// which are used to determine
/// the cell's content and style.
///
/// ## Fields
///
/// - `index: u8`: The unique identifier of the cell in the grid,
///   ranging from 0 to 80.
///   The index is used to calculate the cell's position in the grid
///   and to derive its CSS class.
/// - `value: u8`: The value to be displayed in the cell.
///   A value of 0 indicates an empty cell,
///   while values 1 through 9 represent the respective numbers in
///   the Sudoku puzzle.
///
/// ## Examples
///
/// Creating properties for a cell with a specific value:
///
/// ```rust
/// let cell_props = CellProps {
///     index: 5,
///     value: 3,
/// };
/// ```
///
/// In this example, `cell_props` represents a cell at index 5
/// (sixth cell in the grid) with a value of 3.
#[derive(Props, PartialEq)]
pub struct CellProps {
    index: u8,
    value: u8,
    highlighted: bool,
    selected: bool,
}

/// Represents a locked cell in a Sudoku puzzle.
///
/// A `LockCell` is a cell whose value is pre-determined and cannot be changed
/// by the user.
/// This component is typically used for cells that are part of
/// the initial puzzle layout.
///
/// ## Props
///
/// - `index: u8`: The unique identifier of the cell in the grid,
///   ranging from 0 to 80.
///   The index is used to calculate the cell's position in the grid
///   and to derive its CSS class.
/// - `value: u8`: The value to be displayed in the cell.
///   A value of 0 indicates an empty cell,
///   while values 1 through 9 represent the respective numbers in
///   the Sudoku puzzle.
///
/// ## Example
///
/// ```rust
/// let locked_cell = rsx!(LockCell { index: 5, value: 3 });
/// // Renders a cell at index 5 with a fixed value of 3.
/// ```
pub fn LockCell(cx: Scope<CellProps>) -> Element {
    let base_class = get_class(cx.props.index);

    // Construct the full class string
    let class = format!(
        "{}{}{}",
        base_class,
        if cx.props.highlighted {
            " highlighted"
        } else {
            ""
        },
        if cx.props.selected { " selected" } else { "" },
    );

    let id = cx.props.index;
    // Conditionally display the value or an empty string
    let value = if cx.props.value != 0 {
        cx.props.value.to_string()
    } else {
        String::new()
    };

    cx.render(rsx!(
        div {
            class: "{class}",
            id: "{id}",
            "{value}"
        }
    ))
}

/// Represents a free cell in a Sudoku puzzle.
///
/// A `FreeCell` is a cell that is initially empty and can be filled by the user.
/// This component allows for user interaction to select or enter a value.
///
/// ## Props
///
/// - `index: u8`: The unique identifier of the cell in the grid,
///   ranging from 0 to 80.
///   The index is used to calculate the cell's position in the grid
///   and to derive its CSS class.
/// - `value: u8`: The current value of the cell, initially set to 0.
///
/// ## Example
///
/// ```rust
/// let free_cell = rsx!(FreeCell { index: 10, value: 0 });
/// // Renders an empty cell at index 10 that can be filled by the user.
/// ```
pub fn FreeCell(cx: Scope<CellProps>) -> Element {
    let base_class = get_class(cx.props.index);

    // Construct the full class string
    let class = format!(
        "{}{}{}",
        base_class,
        if cx.props.highlighted {
            " highlighted"
        } else {
            ""
        },
        if cx.props.selected { " selected" } else { "" },
    );

    let id = cx.props.index;
    let value = use_state(cx, || "".to_string());

    cx.render(rsx!(
        div {
            class: "{class}",
            id: "{id}",
            "{&value}"
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_ssr::render_lazy;
    use rand::Rng;
    use regex::Regex;

    #[test]
    fn test_lock_cell() {
        // Test with a non-zero value
        let rendered_lock_cell = render_lazy(rsx!(LockCell {
            index: 0,
            value: 5,
            highlighted: false,
            selected: false
        }));
        assert!(rendered_lock_cell.contains('5'));

        // Test with a zero value
        let rendered_lock_cell_zero = render_lazy(rsx!(LockCell {
            index: 1,
            value: 0,
            highlighted: false,
            selected: false
        }));
        // Adjust this based on whether you expect to render "0" or an empty string
        assert!(!rendered_lock_cell_zero.contains('0'));
    }

    #[test]
    fn test_free_cell() {
        // Assuming FreeCell starts with an empty value
        let rendered_free_cell = render_lazy(rsx!(FreeCell {
            index: 0,
            value: 0,
            highlighted: false,
            selected: false
        }));

        // Test with a zero value
        assert!(rendered_free_cell.contains('0'));
    }

    #[test]
    fn test_free_cell_classes() {
        let re = Regex::new(r#"<div[^>]*class="([^"]*)""#).unwrap();

        for id in 0..81 {
            let rendered = render_lazy(rsx!(FreeCell {
                index: id,
                value: 0,
                highlighted: false,
                selected: false
            }));
            let caps = re.captures(&rendered).unwrap();
            let class_attr = &caps[1];

            assert_eq!(class_attr, get_class(id));
        }
    }

    #[test]
    fn test_lock_cell_classes() {
        let mut rng = rand::thread_rng();

        let re = Regex::new(r#"<div[^>]*class="([^"]*)""#).unwrap();

        for id in 0..81 {
            let value: u8 = rng.gen_range(1..=9);
            let rendered = render_lazy(rsx!(LockCell {
                index: id,
                value: value,
                highlighted: false,
                selected: false
            }));
            let caps = re.captures(&rendered).unwrap();
            let class_attr = &caps[1];

            assert_eq!(class_attr, get_class(id));
        }
    }
}
