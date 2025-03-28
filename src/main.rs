use std::{collections::HashMap};
use extinGUIsh::show_output;

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

    println!("{:?}", w_idx);

    // fire is represented by a true cell, tree by a false cell, ground by a None cell
    let mut state: Vec<Option<bool>> = Vec::from([Some(false), Some(false), None, None, None, None,
            None, Some(true), None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, Some(true), None]);
    
    // probability reduction due to moisture. Really only checked for cell where there are trees
    let moisture: Vec<f32> = Vec::from([0.2, 0.1, 0.3, 0.2, 0.1, 0.0,
    0.2, 0.1, 0.3, 0.2, 0.1, 0.0,0.2, 0.1, 0.3, 0.2, 0.1, 0.0,0.2, 0.1, 0.3, 0.2, 0.1, 0.0,
    0.2, 0.1, 0.3, 0.2, 0.1, 0.0]);

    // indices of only those cells where there is currently a fire
    let fire_indices: Vec<[usize;2]> = Vec::from([[1,1], [4,4]]);

    show_output(state, N, M)

}
