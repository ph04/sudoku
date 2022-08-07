pub struct Sudoku {
    grid: [u8; 81],
    is_solved: bool,
}

impl Sudoku {
    fn is_row_valid(&self, number: u8, row_index: usize) -> bool {
        !self.grid
            .chunks_exact(9)
            .nth(row_index)
            .unwrap()
            .iter()
            .any(|n| *n == number)
    }

    fn is_column_valid(&self, number: u8, col_index: usize) -> bool {
        !self.grid
            .chunks_exact(9)
            .map(|row| row[col_index])
            .any(|n| n == number)
    }

    fn is_cell_valid(&self, number: u8, (x, y): (usize, usize)) -> bool {
        !self.grid
            .chunks_exact(27)
            .nth(y / 3)
            .unwrap()
            .chunks_exact(9)
            .nth(x / 3)
            .unwrap()
            .iter()
            .any(|n| *n == number)
    }

    fn is_valid(&self, number: u8, (x, y): (usize, usize)) -> bool {
        self.is_row_valid(number, y) && 
        self.is_column_valid(number, x) &&
        self.is_cell_valid(number, (x, y))
    }

    fn get_coordinates(mut curr_idx: usize) -> (usize, usize) {
        let mut curr_y = 0;

        while curr_idx >= 9 {
            curr_idx -= 9;

            curr_y += 1;
        }

        (curr_idx, curr_y)
    }

    fn solve_internals(&mut self, curr_idx: usize) {
        println!("{}", self);

        if self.is_solved {
            return
        }

        if curr_idx == 81 {
            self.is_solved = true;
            return
        }

        if self.grid[curr_idx] != 0 {
            self.solve_internals(curr_idx + 1);
        } else {
            for n in 1..=9 {
                if self.is_valid(n, Self::get_coordinates(curr_idx)) {
                    self.grid[curr_idx] = n;

                    self.solve_internals(curr_idx + 1);
                }
            }

            self.grid[curr_idx] = 0;
        }
    }

    pub fn solve(&mut self) {
        self.solve_internals(0);
    }
}

impl TryFrom<[u8; 81]> for Sudoku {
    type Error = SudokuError;

    fn try_from(grid: [u8; 81]) -> Result<Self, Self::Error> {
        if grid.iter().any(|n| *n > 9) {
            Err(SudokuError::InvalidCell)
        } else {
            Ok(Self { grid, is_solved: false })
        }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}",
            self.grid
                .chunks_exact(9)
                .map(|row| {
                    row
                        .iter()
                        .map(|n| n.to_string())
                        .intersperse(' '.to_string())
                        .collect::<String>()
                })
                .intersperse('\n'.to_string())
                .collect::<String>()
        )
    }
}

#[derive(Debug)]
pub enum SudokuError {
    InvalidCell
}

impl std::fmt::Display for SudokuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidCell => writeln!(f, "There are invalid cells in the given sudoku."),
        }
    }
}

impl std::error::Error for SudokuError {}
