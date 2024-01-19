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

pub fn create_sudoku() -> [u8; 81] {
    Sudoku::generate().to_bytes()
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
}
