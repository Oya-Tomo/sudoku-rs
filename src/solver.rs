pub type Board = Vec<Vec<u8>>;

pub struct Solver {
    pub board: Board,
    blanks: Vec<(usize, usize)>,
    pub answers: Vec<Board>,
}

impl Solver {
    pub fn new(board: Board) -> Self {
        let mut blanks = Vec::new();
        for (i, row) in board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    blanks.push((i, j));
                }
            }
        }
        Self {
            board,
            blanks,
            answers: Vec::new(),
        }
    }

    pub fn solve(&mut self) {
        self.search(0);
    }

    pub fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..9 {
            if self.board[row][i] == num || self.board[i][col] == num {
                return false;
            }
        }

        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn search(&mut self, idx: usize) {
        if idx == self.blanks.len() {
            self.answers.push(self.board.clone());
            return;
        }

        let (row, col) = self.blanks[idx];
        for num in 1..=9 {
            if self.is_valid(row, col, num) {
                self.board[row][col] = num;
                self.search(idx + 1);
                self.board[row][col] = 0;
            }
        }
    }
}
