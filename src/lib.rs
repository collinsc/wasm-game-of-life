use wasm_bindgen::prelude::*;
use js_sys::Math::random;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;


#[wasm_bindgen]
pub enum CreationStrategy {
    Deterministic,
    Spaceship,
    FiftyFifty,
    Empty
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new(strategy: CreationStrategy) -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;

        let size = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(size);
            
        let mut universe = Universe {
            width,
            height,
            cells,
        };
        universe.init(strategy);
        universe
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.clear();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.clear();
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }


    fn clear (&mut self) {
        let size = (self.width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    fn init(&mut self, strategy: CreationStrategy) {
        for i in 0..self.width * self.height {
            let state = match strategy {
                CreationStrategy::Deterministic => {
                    i % 2 == 0 || i % 7 == 0
                },
                CreationStrategy::Spaceship => {
                    let r_delta = self.height / 2 - (i / self.width);
                    let c_delta = self.width / 2 - (i % self.width);
                    match (r_delta,c_delta) {
                        (0,0) |                (0,3) |
                                                       (1,4) |
                        (2,0) |                        (2,4) | 
                               (3,1) | (3,2) | (3,3) | (3,4)   => true,
                        (_,_) => false,
                    }
                },
                CreationStrategy::FiftyFifty => {
                    let rand = random();
                    if rand > 0.5 {
                        true
                    } else {
                        false
                    }
                },
                _ => false
            };
            self.cells.set(i as usize, state);
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }


    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}