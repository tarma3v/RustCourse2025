#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let index = row * self.cols + col;
        &self.grid[index]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        let index = row * self.cols + col;
        self.grid[index] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                if new_row >= 0
                    && new_row < self.rows as isize
                    && new_col >= 0
                    && new_col < self.cols as isize
                {
                    neighbours.push((new_row as usize, new_col as usize));
                }
            }
        }
        neighbours
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let (rows, cols) = self.grid.size();
        let mut next_grid = Grid::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                let current_cell = self.grid.get(row, col);
                let live_neighbors = self
                    .grid
                    .neighbours(row, col)
                    .into_iter()
                    .filter(|&(r, c)| *self.grid.get(r, c) == Cell::Alive)
                    .count();

                let next_cell = match current_cell {
                    Cell::Alive => {
                        if !(2..=3).contains(&live_neighbors) {
                            Cell::Dead
                        } else {
                            Cell::Alive
                        }
                    }
                    Cell::Dead => {
                        if live_neighbors == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                };
                next_grid.set(next_cell, row, col);
            }
        }
        self.grid = next_grid;
    }
}
