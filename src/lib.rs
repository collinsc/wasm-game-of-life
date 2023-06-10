use wasm_bindgen::prelude::*;
use js_sys::Math::random;

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
        let shape: &[(i8,i8,bool)] = match to_draw {
            DrawObject::Spaceship => &SPACESHIP,
            DrawObject::Glider => &GLIDER,
            DrawObject::Pulsar => &PULSAR,
        };

        for (delta_r, delta_c, is_alive) in shape {
            let idx = self.get_delta_index(row, col, *delta_r, *delta_c);
            set_cell(&mut self.cells, idx, *is_alive);
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

const SPACESHIP: [(i8,i8,bool); 13 * 10]= [
                                          (-6,-4, false),(-6,-3, false),(-6,-2, false),(-6,-1, false),(-6, 0,  true),(-6, 1,  true),(-6, 2, false),(-6, 3, false),(-6, 4, false),(-6, 5, false),
                                          (-5,-4, false),(-5,-3, false),(-5,-2, false),(-5,-1,  true),(-5, 0,  true),(-5, 1,  true),(-5, 2,  true),(-5, 3, false),(-5, 4, false),(-5, 5, false),
                                          (-4,-4, false),(-4,-3, false),(-4,-2, false),(-4,-1, false),(-4, 0, false),(-4, 1, false),(-4, 2, false),(-4, 3, false),(-4, 4, false),(-4, 5, false),
                                          (-3,-4, false),(-3,-3, false),(-3,-2,  true),(-3,-1,  true),(-3, 0,  true),(-3, 1,  true),(-3, 2,  true),(-3, 3,  true),(-3, 4, false),(-3, 5, false),
                                          (-2,-4, false),(-2,-3, false),(-2,-2, false),(-2,-1,  true),(-2, 0,  true),(-2, 1,  true),(-2, 2,  true),(-2, 3, false),(-2, 4, false),(-2, 5, false),
                                          (-1,-4, false),(-1,-3, false),(-1,-2, false),(-1,-1, false),(-1, 0, false),(-1, 1, false),(-1, 2, false),(-1, 3, false),(-1, 4, false),(-1, 5, false),
                                          ( 0,-4, false),( 0,-3, false),( 0,-2,  true),( 0,-1,  true),( 0, 0, false),( 0, 1, false),( 0, 2,  true),( 0, 3,  true),( 0, 4, false),( 0, 5, false),
                                          ( 1,-4,  true),( 1,-3,  true),( 1,-2, false),( 1,-1,  true),( 1, 0, false),( 1, 1, false),( 1, 2,  true),( 1, 3, false),( 1, 4,  true),( 1, 5,  true),
                                          ( 2,-4, false),( 2,-3, false),( 2,-2, false),( 2,-1,  true),( 2, 0, false),( 2, 1, false),( 2, 2,  true),( 2, 3, false),( 2, 4, false),( 2, 5, false),
                                          ( 3,-4, false),( 3,-3, false),( 3,-2, false),( 3,-1, false),( 3, 0, false),( 3, 1, false),( 3, 2, false),( 3, 3, false),( 3, 4, false),( 3, 5, false),
                                          ( 4,-4, false),( 4,-3, false),( 4,-2, false),( 4,-1, false),( 4, 0, false),( 4, 1, false),( 4, 2, false),( 4, 3, false),( 4, 4, false),( 4, 5, false),
                                          ( 5,-4, false),( 5,-3, false),( 5,-2, false),( 5,-1, false),( 5, 0,  true),( 5, 1,  true),( 5, 2, false),( 5, 3, false),( 5, 4, false),( 5, 5, false),
                                          ( 6,-4, false),( 6,-3, false),( 6,-2, false),( 6,-1, false),( 6, 0,  true),( 6, 1,  true),( 6, 2, false),( 6, 3, false),( 6, 4, false),( 6, 5, false),
                                          ];
const PULSAR: [(i8,i8,bool); 13 * 13] = [
                                          (-6,-6, false),(-6,-5, false),(-6,-4,  true),(-6,-3,  true),(-6,-2,  true),(-6,-1, false),(-6, 0, false),(-6, 1, false),(-6, 2,  true),(-6, 3,  true),(-6, 4,  true),(-6, 5, false),(-6, 6, false),
                                          (-5,-6, false),(-5,-5, false),(-5,-4, false),(-5,-3, false),(-5,-2, false),(-5,-1, false),(-5, 0, false),(-5, 1, false),(-5, 2, false),(-5, 3, false),(-5, 4, false),(-5, 5, false),(-5, 6, false),
                                          (-4,-6,  true),(-4,-5, false),(-4,-4, false),(-4,-3, false),(-4,-2, false),(-4,-1,  true),(-4, 0, false),(-4, 1,  true),(-4, 2, false),(-4, 3, false),(-4, 4, false),(-4, 5, false),(-4, 6,  true),
                                          (-3,-6,  true),(-3,-5, false),(-3,-4, false),(-3,-3, false),(-3,-2, false),(-3,-1,  true),(-3, 0, false),(-3, 1,  true),(-3, 2, false),(-3, 3, false),(-3, 4, false),(-3, 5, false),(-3, 6,  true),
                                          (-2,-6,  true),(-2,-5, false),(-2,-4, false),(-2,-3, false),(-2,-2, false),(-2,-1,  true),(-2, 0, false),(-2, 1,  true),(-2, 2, false),(-2, 3, false),(-2, 4, false),(-2, 5, false),(-2, 6,  true),
                                          (-1,-6, false),(-1,-5, false),(-1,-4,  true),(-1,-3,  true),(-1,-2,  true),(-1,-1, false),(-1, 0, false),(-1, 1, false),(-1, 2,  true),(-1, 3,  true),(-1, 4,  true),(-1, 5, false),(-1, 6, false),
                                          ( 0,-6, false),( 0,-5, false),( 0,-4, false),( 0,-3, false),( 0,-2, false),( 0,-1, false),( 0, 0, false),( 0, 1, false),( 0, 2, false),( 0, 3, false),( 0, 4, false),( 0, 5, false),( 0, 6, false),
                                          ( 1,-6, false),( 1,-5, false),( 1,-4,  true),( 1,-3,  true),( 1,-2,  true),( 1,-1, false),( 1, 0, false),( 1, 1, false),( 1, 2,  true),( 1, 3,  true),( 1, 4,  true),( 1, 5, false),( 1, 6, false),
                                          ( 2,-6,  true),( 2,-5, false),( 2,-4, false),( 2,-3, false),( 2,-2, false),( 2,-1,  true),( 2, 0, false),( 2, 1,  true),( 2, 2, false),( 2, 3, false),( 2, 4, false),( 2, 5, false),( 2, 6,  true),
                                          ( 3,-6,  true),( 3,-5, false),( 3,-4, false),( 3,-3, false),( 3,-2, false),( 3,-1,  true),( 3, 0, false),( 3, 1,  true),( 3, 2, false),( 3, 3, false),( 3, 4, false),( 3, 5, false),( 3, 6,  true),
                                          ( 4,-6,  true),( 4,-5, false),( 4,-4, false),( 4,-3, false),( 4,-2, false),( 4,-1,  true),( 4, 0, false),( 4, 1,  true),( 4, 2, false),( 4, 3, false),( 4, 4, false),( 4, 5, false),( 4, 6,  true),
                                          ( 5,-6, false),( 5,-5, false),( 5,-4, false),( 5,-3, false),( 5,-2, false),( 5,-1, false),( 5, 0, false),( 5, 1, false),( 5, 2, false),( 5, 3, false),( 5, 4, false),( 5, 5, false),( 5, 6, false),
                                          ( 6,-6, false),( 6,-5, false),( 6,-4,  true),( 6,-3,  true),( 6,-2,  true),( 6,-1, false),( 6, 0, false),( 6, 1, false),( 6, 2,  true),( 6, 3,  true),( 6, 4,  true),( 6, 5, false),( 6, 6, false),
                                          ];
const GLIDER : [(i8,i8,bool); 9] = [ 
                                    (-1,-1, false),(-1, 0, false),(-1, 1,  true),
                                    ( 0,-1,  true),( 0, 0, false),( 0, 1,  true),
                                    ( 1,-1, false),( 1, 0,  true),( 1, 1,  true),
                                    ];




impl Universe {
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn get_delta_index(&self, row: u32, col: u32, delta_row: i8, delta_col: i8) -> usize {
        let r_del: u32  = if delta_row < 0 { 
            (delta_row + (self.height as i8)) as u32
        } else {
            delta_row as u32
        };
        let _row = (row + r_del) % self.height;
        let c_del:u32 = if delta_col < 0 {
            (delta_col + (self.width as i8)) as u32
        } else {
            delta_col as u32
        };
        let _col = (col + c_del) % self.width;
        self.get_index(_row, _col)
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

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for (delta_row, delta_col, _) in GLIDER {
            if delta_row == 0 && delta_col == 0 {
                continue;
            }
            let idx = self.get_delta_index(row, column, delta_row, delta_col);
            let is_alive = get_cell(&self.cells, idx);
            if is_alive {
                count += 1;
            }
        }
        count
    }
}