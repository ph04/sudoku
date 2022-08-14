use std::fmt::Write;

/// Represents a sudoku grid. The grid is represented
/// using a [u8; 81] array, and the blank cells are represented with `0`s.
#[derive(Debug, Clone, Copy)]
pub struct Sudoku {
    grid: [u8; 81],
    is_solved: bool,
}

impl Sudoku {
    /// Flattens a given `[[u8; 9]; 9]` into a `[u8; 81]`.
    fn flatten(input_grid: [[u8; 9]; 9]) -> [u8; 81] {
        let mut grid = [0; 81];

        grid
            .chunks_exact_mut(9)
            .zip(input_grid.iter())
            .for_each(|(row, input_row)| {
                row
                    .iter_mut()
                    .zip(input_row.iter())
                    .for_each(|(r, ir)| *r = *ir);
            });

        grid
    }

    /// Checks if the given number can be put in the given row.
    fn is_row_valid(&self, number: u8, row_index: usize) -> bool {
        !self.grid
            .chunks_exact(9)
            .nth(row_index)
            .unwrap()
            .iter()
            .any(|n| *n == number)
    }

    /// Checks if the given number can be put in the given column.
    fn is_column_valid(&self, number: u8, col_index: usize) -> bool {
        !self.grid
            .chunks_exact(9)
            .map(|row| row[col_index])
            .any(|n| n == number)
    }

    /// Checks if the given number can be put in the given cell.
    fn is_cell_valid(&self, number: u8, (x, y): (usize, usize)) -> bool {
        !self.grid
            .chunks_exact(27)
            .nth(y / 3)
            .unwrap()
            .chunks_exact(9)
            .any(|row| {
                row
                    .chunks_exact(3)
                    .nth(x / 3)
                    .unwrap()
                    .iter()
                    .any(|n| *n == number)
            })
    }

    /// Checks if the given number can be put in the given position.
    fn is_valid(&self, number: u8, (x, y): (usize, usize)) -> bool {
        self.is_row_valid(number, y) && self.is_column_valid(number, x) && self.is_cell_valid(number, (x, y))
    }

    /// Transforms the given index into a coordinates pair.
    fn get_coordinates(curr_idx: usize) -> (usize, usize) {
        (curr_idx % 9, curr_idx / 9)
    }

    /// Recursively solves the sudoku through backtracking,
    /// starting from the given index.
    fn solve_internals(&mut self, curr_idx: usize) {
        if curr_idx == 81 {
            self.is_solved = true;
            return;
        }

        if self.grid[curr_idx] != 0 {
            self.solve_internals(curr_idx + 1);
        } else {
            for n in 1..=9 {
                if self.is_valid(n, Self::get_coordinates(curr_idx)) {
                    self.grid[curr_idx] = n;

                    self.solve_internals(curr_idx + 1);

                    if self.is_solved {
                        return;
                    }
                }
            }

            self.grid[curr_idx] = 0;
        }
    }

    /// Solves the sudoku by using recursion and backtracking.
    ///
    /// # Example
    ///
    /// ```
    /// # use sudoku::sudoku::Sudoku;
    /// let mut sudoku = Sudoku::default(); // empty grid
    /// 
    /// sudoku.solve() // the sudoku is now solved!
    /// ```
    pub fn solve(&mut self) {
        self.solve_internals(0);
    }

    /// Returns the current state of the sudoku.
    ///
    /// # Example
    ///
    /// ```
    /// # use sudoku::sudoku::Sudoku;
    /// let sudoku = Sudoku::default(); // blank grid
    /// 
    /// assert_eq!(sudoku.grid(), [0; 81]);
    /// ```
    pub fn grid(&self) -> [u8; 81] {
        self.grid
    }
}

impl TryFrom<[u8; 81]> for Sudoku {
    type Error = SudokuError;

    fn try_from(grid: [u8; 81]) -> Result<Self, Self::Error> {
        if grid.iter().any(|n| *n > 9) {
            Err(SudokuError::InvalidCell)
        } else {
            Ok(Self {
                grid,
                is_solved: false
            })
        }
    }
}

impl TryFrom<[[u8; 9]; 9]> for Sudoku {
    type Error = SudokuError;

    fn try_from(input_grid: [[u8; 9]; 9]) -> Result<Self, Self::Error> {
        if input_grid.iter().any(|row| row.iter().any(|n| *n > 9)) {
            Err(SudokuError::InvalidCell)
        } else {
            Ok(Self {
                grid: Self::flatten(input_grid),
                is_solved: false,
            })
        }

    }
}

impl std::cmp::PartialEq<[u8; 81]> for Sudoku {
    fn eq(&self, other: &[u8; 81]) -> bool {
        self.grid() == *other
    }
}

impl std::cmp::PartialEq<[[u8; 9]; 9]> for Sudoku {
    fn eq(&self, other: &[[u8; 9]; 9]) -> bool {
        *self == Self::flatten(*other)
    }
}

impl Default for Sudoku {
    /// Returns an empty grid.
    fn default() -> Self {
        Self {
            grid: [0; 81],
            is_solved: false,
        }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid
            .chunks_exact(9)
            .flat_map(|row|
                row
                    .iter()
                    .map(|n| if *n != 0 { char::from_digit(*n as u32, 10).unwrap() } else { '_' })
                    .intersperse(' ')
                    .chain(Some('\n'))
            )
            .try_for_each(|c| f.write_char(c))
    }
}

/// Error enum used to return errors when invalid grids are given.
#[derive(Debug)]
pub enum SudokuError {
    /// The case when there is a number greater than 9.
    InvalidCell
}

impl std::fmt::Display for SudokuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidCell => writeln!(f, "There are invalid cells (greater than 9) in the given sudoku."),
        }
    }
}

impl std::error::Error for SudokuError {}
