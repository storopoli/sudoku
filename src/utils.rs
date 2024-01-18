use sudoku::board::Sudoku;

pub fn create_sudoku() -> [u8; 81] {
    Sudoku::generate().to_bytes()
}
