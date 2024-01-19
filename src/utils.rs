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
