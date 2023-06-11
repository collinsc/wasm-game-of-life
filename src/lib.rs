use wasm_bindgen::prelude::*;
use js_sys::Math::random;
mod shapes;
use crate::shapes::Shape;

#[wasm_bindgen]
pub enum CreationStrategy {
    Deterministic,
    FiftyFifty,
    Empty
}

#[wasm_bindgen]
pub enum DrawObject {
    Spaceship,
    Glider,
    Pulsar,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
    _buff: Vec<u8>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let cells = Vec::new();
        let _buff = Vec::new();
        let mut uni = Universe {
            width,
            height,
            cells,
            _buff
        };
        uni.clear();
        uni
    }

    pub fn tick(&mut self) {

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = get_cell(&self.cells, idx);
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
                set_cell(&mut self._buff, idx, next_cell);
            }
        }

        for idx in 0..self._buff.len() {
            self.cells[idx] = self._buff[idx];    
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cell_ptr(&self) -> *const u8{
        return self.cells.as_ptr();
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

    pub fn init(&mut self, strategy: CreationStrategy) {
        for i in 0..self.width * self.height {
            let state = match strategy {
                CreationStrategy::Deterministic => {
                    i % 2 == 0 || i % 7 == 0
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
            set_cell(&mut self.cells, i as usize, state);
        }
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        let is_alive = get_cell(&self.cells, idx);
        set_cell(&mut self.cells, idx, !is_alive);
    }

    pub fn draw_object(&mut self, to_draw:DrawObject, row: u32, col: u32) {
        let shape: &'static Shape = match to_draw {
            DrawObject::Spaceship => &shapes::SPACESHIP,
            DrawObject::Glider => &shapes::GLIDER,
            DrawObject::Pulsar => &shapes::PULSAR,
        };
        for idx in 0..shape.width * shape.height {
            let idx_alive_tup = self.translate_idx(&shape, row, col, idx);
            set_cell(&mut self.cells, idx_alive_tup.0, idx_alive_tup.1);
        }
    }
}

fn get_cell(arr: &Vec<u8>, idx: usize) -> bool {
    arr[idx / 8] & (1 << (idx % 8)) != 0
}

fn set_cell(arr: &mut Vec<u8>, idx: usize, is_alive: bool) {
    if is_alive {
        arr[idx / 8] |= 1 << (idx % 8)
    } else {
        arr[idx / 8] &= !(1 << (idx % 8))
    }
}



impl Universe {
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn translate_idx(&self, shape: &'static Shape, ctr_row:u32, ctr_col:u32, shape_idx: u32) ->(usize, bool) {
        let row_shift = (shape_idx / shape.width) as i32 + shape.row_offset;
        let col_shift = (shape_idx % shape.width) as i32 + shape.col_offset;
        let row_offset = if row_shift < 0 {
            self.height() as i32 + row_shift
        } else { 
            row_shift 
        } as u32;
        let col_offset = if col_shift < 0 {
            self.width() as i32 + col_shift
        } else {
            col_shift
        } as u32;
        let cell_row = (ctr_row + row_offset) % self.height();
        let cell_col = (ctr_col + col_offset) % self.width();
        let draw_idx = self.get_index(cell_row, cell_col);
        let is_alive = shape.pattern[(shape_idx / 8) as usize] & (128 >> (shape_idx % 8)) != 0;
        (draw_idx, is_alive)
    }


    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &Vec<u8> {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            set_cell(&mut self.cells, idx, true);
        }
    }

    fn clear (&mut self) {
        let size = ((self.width * self.height) as f32 / 8.0).ceil() as usize ;
        self.cells = (0..size).map(|_| 0).collect();
        self._buff = (0..size).map(|_| 0).collect();
    }

    pub fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let shape = &shapes::NEIGHBORS;
        let mut count = 0;
        for idx in 0..shape.width * shape.height {
            let idx_alive_tup = self.translate_idx(&shape, row, column, idx);
            if idx_alive_tup.1 && get_cell(&self.cells, idx_alive_tup.0) {
                count += 1;
            }
        }
        count
    }
}