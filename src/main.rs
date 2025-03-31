use std::{collections::HashMap, env::current_exe, io::{stdout, Write}};
use extinGUIsh::{show_output, change_states, check_fire};

fn main() {
    const N: usize = 5; // number of rows in grid
    const M: usize = 6; // number of columns in grid

    let p_base: f32 = 0.4; // base probability for a tree catching fire
    let w_boost: f32 = 0.2; // probability boost due to wind
    
    // mapping of wind direction to needed change in indices from tree to fire for a probability boost
    let directions: HashMap<char, [i8; 2]> = HashMap::from(
        [('N',[-1,0]),('W',[0,-1]),('S',[1,0]),('E',[0,1])]
    ); 
    let w_direction: char = 'S'; // current wind direction
    let w_idx: [i8; 2] = directions[&w_direction]; // current wind direction -> change in indices

    // fire is represented by a true cell, tree by a false cell, ground by a None cell
    let mut state: Vec<Option<bool>> = Vec::from([Some(false), Some(false), None, None, None, None,
            None, Some(true), Some(false), None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, Some(true), None]);
    
    // probability reduction due to moisture. Really only checked for cell where there are trees
    let moisture: Vec<f32> = Vec::from([0.2, 0.1, 0.3, 0.2, 0.1, 0.0,
    0.2, 0.1, 0.3, 0.2, 0.1, 0.0,0.2, 0.1, 0.3, 0.2, 0.1, 0.0,0.2, 0.1, 0.3, 0.2, 0.1, 0.0,
    0.2, 0.1, 0.3, 0.2, 0.1, 0.0]);

    // indices of only those cells where there is currently a fire
    let mut fire_indices: Vec<usize> = Vec::from([1*M + 1, 4*M + 4]);
    
    // to-do: TEST IF RNG GENERATION IS IMPLEMENTED CORRECTLY
    let mut rng: rand::prelude::ThreadRng = rand::rng(); 

    
    // simulation shows output and ends when there is no fire in grid. Otherwise, shows output and calculates new wildfire positions
    while check_fire(&state) {
        show_output(&&state, N, M);
        fire_indices = change_states(&mut state, &directions, &fire_indices, p_base, &w_idx, w_boost, &moisture, N, M, &mut rng);
    }
    show_output(&&state, N, M);    

}
