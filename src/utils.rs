//! # Utils Module
//!
//! The `utils` module provides utility functions and helpers used across
//! the Sudoku game. It includes functionality such as puzzle generation,
//! solving algorithms, and other common tasks.
//!
//! These utilities are meant to be helper functions that can be used by
//! different parts of the application to perform common operations
//! or calculations.

use sudoku::board::Sudoku;

/// Generates a new Sudoku puzzle.
///
/// This function creates a complete 9x9 Sudoku puzzle. Each Sudoku puzzle
/// is generated randomly and returned as a flat array of 81 `u8` values,
/// representing the puzzle's cells.
/// In this array, each value corresponds to a cell in the Sudoku grid,
/// ordered row by row from top-left to bottom-right.
///
/// The values in the array range from 1 to 9,
/// corresponding to the filled cells in the puzzle.
/// A value of 0 indicates an empty cell that players need to fill in.
///
/// ## Returns
///
/// Returns a `[u8; 81]`, which represents a 9x9 Sudoku puzzle.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
/// let puzzle = create_sudoku();
/// ```
pub fn create_sudoku() -> [u8; 81] {
    Sudoku::generate().to_bytes()
}

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
pub fn get_class(id: u8) -> &'static str {
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
pub fn get_related_cells(index: u8) -> Vec<u8> {
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

/// Updates a class string by adding or removing a specified class.
/// Useful for CSS class changes in DOM Elements.
///
/// This function takes an existing class string (`base_class`), a specific class
/// to add or remove (`class`), and a boolean flag (`add`) that indicates whether
/// to add or remove the class. It then updates the class string accordingly.
///
/// If `add` is `true` and the class is not already in the `base_class` string,
/// the function appends it. If `add` is `false` and the class is present,
/// the function removes it.
/// The function also ensures that there are no leading or trailing whitespaces
/// and that classes are separated by a single space.
///
/// ## Parameters
///
/// - `base_class: String`: The original class string.
/// - `class: String`: The class to add or remove.
/// - `add: bool`: A boolean flag indicating whether to add (`true`) or remove (`false`) the class.
///
/// ## Returns
///
/// Returns the updated class string.
///
/// ## Examples
///
/// Adding a class:
///
/// ```rust
/// let class_string = "some-class another-class".to_string();
/// let updated = update_class(class_string, "selected".to_string(), true);
/// ```
///
/// Removing a class:
///
/// ```rust
/// let class_string = "some-class another-class selected".to_string();
/// let updated = update_class(class_string, "selected".to_string(), false);
/// ```
pub fn update_class(mut base_class: String, class: String, add: bool) -> String {
    let contains_class = base_class.split_whitespace().any(|word| word == class);

    if add && !contains_class {
        // Add the class if it's not present and needs to be added
        base_class.push(' '); // Ensure there's a space before adding
        base_class.push_str(&class);
    } else if !add && contains_class {
        // Remove the class if it's present and needs to be removed
        base_class = base_class
            .split_whitespace()
            .filter(|&word| word != class)
            .collect::<Vec<_>>()
            .join(" ");
    }

    base_class.trim().to_string() // Trim whitespace and return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sudoku_length() {
        for _ in 0..100 {
            let sudoku = create_sudoku();
            assert_eq!(sudoku.len(), 81);
        }
    }

    #[test]
    fn test_create_sudoku_values() {
        for _ in 0..100 {
            let sudoku = create_sudoku();
            assert!(sudoku.iter().all(|&val| (0..=9).contains(&val)));
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

    #[test]
    fn test_update_class() {
        assert_eq!(
            update_class(
                "some-class another-class".to_string(),
                "selected".to_string(),
                true
            ),
            "some-class another-class selected"
        );
        assert_eq!(
            update_class(
                "some-class another-class selected".to_string(),
                "selected".to_string(),
                false
            ),
            "some-class another-class"
        );
        assert_eq!(
            update_class("".to_string(), "selected".to_string(), true),
            "selected"
        );
        assert_eq!(
            update_class("selected".to_string(), "selected".to_string(), false),
            ""
        );

        // Testing removal from between other classes
        assert_eq!(
            update_class(
                "some-class selected another-class".to_string(),
                "selected".to_string(),
                false
            ),
            "some-class another-class"
        );

        // Testing idempotence
        assert_eq!(
            update_class(
                "some-class selected".to_string(),
                "selected".to_string(),
                true
            ),
            "some-class selected"
        );
        assert_eq!(
            update_class("some-class".to_string(), "selected".to_string(), false),
            "some-class"
        );
    }
}
