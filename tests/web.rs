//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

extern crate collinsc_wasm_game_of_life;
use collinsc_wasm_game_of_life::Universe;


#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(6,6);
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[wasm_bindgen_test]
pub fn test_empty_tick() {
    
    // Let's create an empty Universe  to test!
    let mut input_universe = Universe::new(64, 64);

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = Universe::new(64, 64);

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[wasm_bindgen_test]
pub fn test_toggle() {
    
    // Let's create an empty Universe  to test!
    let mut input_universe = Universe::new(6, 6);

    // This is what our spaceship should look like
    // after one tick in our universe.
    let mut expected_universe = Universe::new(6, 6);
    expected_universe.set_cells(&[(0,0)]);

    input_universe.toggle_cell(0,0);
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
