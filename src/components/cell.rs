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

use dioxus::prelude::*;

/// Returns the CSS class for a Sudoku cell based on its ID.
///
/// The Sudoku board is divided into a 9x9 grid, and each cell is assigned a
/// unique ID from 0 to 80.
/// This function maps each cell's ID to a specific set of CSS classes that
/// determine the cell's appearance.
///
/// ## Parameters
///
/// - `id: u8`: The ID of the cell, ranging from 0 to 80.
///
/// ## Returns
///
/// Returns a `&'static str` representing the CSS class or classes for the cell.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
/// let cell_class = get_class(0);
/// assert_eq!(cell_class, "tsb lsb rdb bdb");
///
/// let cell_class = get_class(10);
/// assert_eq!(cell_class, "bdb");
/// ```
///
/// Note: The returned classes are meant to be used in the context of a web page
/// or a web-based UI renderer.
fn get_class(id: u8) -> &'static str {
    match id {
        0 | 3 | 6 | 27 | 30 | 33 | 54 | 57 | 60 => "tsb lsb rdb bdb",
        1 | 4 | 7 | 28 | 31 | 34 | 55 | 58 | 61 => "tsb  bdb",
        2 | 5 | 29 | 32 | 56 | 59 => "tsb bdb ldb",
        8 | 35 | 62 => "tsb rsb bdb ldb",
        9 | 12 | 15 | 36 | 39 | 42 | 63 | 66 | 69 => "lsb rdb bdb",
        10 | 13 | 16 | 37 | 40 | 43 | 64 | 67 | 70 => "bdb",
        11 | 14 | 38 | 41 | 65 | 68 => "bdb ldb",
        18 | 21 | 24 | 45 | 48 | 51 => "lsb rdb",
        20 | 23 | 47 | 50 => "ldb",
        17 | 44 | 71 => "rsb ldb bdb",
        26 | 53 => "rsb ldb",
        72 | 75 | 78 => "bsb lsb rdb",
        73 | 76 | 79 => "bsb",
        74 | 77 => "bsb ldb",
        80 => "bsb rsb ldb",
        _ => "",
    }
}

/// Calculates the indices of all cells related to a given cell in a Sudoku
/// puzzle.
///
/// In a Sudoku puzzle, a cell is related to all other cells in the same row,
/// the same column, and the same 3x3 sub-square.
/// This function takes the index of a cell and returns a list of indices for
/// all related cells.
///
/// ## Parameters
///
/// - `index: u8`: The index of the cell in the Sudoku grid.
///    Must be in the range 0 to 80.
///
/// ## Returns
///
/// Returns a `Vec<u8>` containing the indices of all cells related to the
/// given cell, excluding the cell itself.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
/// let related_cells = get_related_cells(40); // Center cell of the board
/// let related_cells = get_related_cells(0); // Top-left corner of the board
/// ```
fn get_related_cells(index: u8) -> Vec<u8> {
    let mut related_cells = Vec::new();
    let row = index / 9;
    let col = index % 9;
    let start_row = row / 3 * 3;
    let start_col = col / 3 * 3;

    // Add cells in the same row
    for i in 0..9 {
        related_cells.push(row * 9 + i);
    }

    // Add cells in the same column
    for i in 0..9 {
        related_cells.push(i * 9 + col);
    }

    // Add cells in the same 3x3 sub-grid
    for i in start_row..start_row + 3 {
        for j in start_col..start_col + 3 {
            related_cells.push(i * 9 + j);
        }
    }

    // Remove duplicates and the original cell
    related_cells.sort_unstable();
    related_cells.dedup();
    related_cells.retain(|&x| x != index);

    related_cells
}

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
    let class = get_class(cx.props.index);
    let id = cx.props.index;
    // Conditionally display the value or an empty string
    let value = if cx.props.value != 0 {
        cx.props.value.to_string()
    } else {
        String::new()
    };

    cx.render(rsx!(
        div {
            class: class,
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
    let class = get_class(cx.props.index);
    let id = cx.props.index;
    let value = use_state(cx, || "".to_string());

    cx.render(rsx!(
        div {
            class: class,
            id: "{id}",
            "{value}"
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
        let rendered_lock_cell = render_lazy(rsx!(LockCell { index: 0, value: 5 }));
        assert!(rendered_lock_cell.contains('5'));

        // Test with a zero value
        let rendered_lock_cell_zero = render_lazy(rsx!(LockCell { index: 1, value: 0 }));
        // Adjust this based on whether you expect to render "0" or an empty string
        assert!(!rendered_lock_cell_zero.contains('0'));
    }

    #[test]
    fn test_free_cell() {
        // Assuming FreeCell starts with an empty value
        let rendered_free_cell = render_lazy(rsx!(FreeCell { index: 0, value: 0 }));

        // Test with a zero value
        assert!(rendered_free_cell.contains('0'));
    }

    #[test]
    fn test_free_cell_classes() {
        let re = Regex::new(r#"<div[^>]*class="([^"]*)""#).unwrap();

        for id in 0..81 {
            let rendered = render_lazy(rsx!(FreeCell {
                index: id,
                value: 0
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
            }));
            let caps = re.captures(&rendered).unwrap();
            let class_attr = &caps[1];

            assert_eq!(class_attr, get_class(id));
        }
    }

    #[test]
    fn test_related_cells_middle() {
        let index = 40; // Center cell of the board
        let related = get_related_cells(index);
        assert_eq!(related.len(), 20); // 8 in row, 8 in column, 4 in sub-grid, excluding duplicates and the cell itself
        assert!(related.contains(&32)); // Another cell in the same sub-grid
        assert!(related.contains(&39)); // Another cell in the same row
        assert!(related.contains(&49)); // Another cell in the same column
    }

    #[test]
    fn test_related_cells_corner() {
        let index = 0; // Top-left corner cell of the board
        let related = get_related_cells(index);
        assert_eq!(related.len(), 20); // 8 in row, 8 in column, 4 in sub-grid, excluding duplicates and the cell itself
        assert!(related.contains(&1)); // Another cell in the same row
        assert!(related.contains(&9)); // Another cell in the same column
        assert!(related.contains(&10)); // Another cell in the same sub-grid
    }

    #[test]
    fn test_related_cells_edge() {
        let index = 9; // An edge cell (but not corner)
        let related = get_related_cells(index);
        assert_eq!(related.len(), 20); // 8 in row, 8 in column, 4 in sub-grid, excluding duplicates and the cell itself
        assert!(related.contains(&0)); // Another cell in the same column
        assert!(related.contains(&11)); // Another cell in the same row
        assert!(related.contains(&20)); // Another cell in the same sub-grid
    }
}
