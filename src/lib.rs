use std::collections::HashMap;
use num::clamp;
use rand::Rng;

/// Used to output grid to command line interface.
/// * `state` - the grid itself
/// * `N` - number of rows in grid
/// * `M` - number of columns in grid
pub fn show_output(state: &Vec<Option<bool>>, N: usize, M: usize){

    for i in 0..N {
        for j in 0..M {
            if state[M*i + j] == Some(true){
                print!("F ");
            }
            else if state[M*i + j] == Some(false){
                print!("T ");
            }
            else{
                print!(". ");
            }
            
        }
        println!();
    }
    println!();

}

/// Calculates and returns probability that a certain tree will catch fire from the nearby fire cell.
/// * `p_base` - base probability without any modifiers
/// * `fire_idx` - (flattened) index of the relevant cell with fire
/// * `tree_idx` - (flattened) index of the relevant cell with tree
/// * `wind` - needed operations on i and j index to see whether there will be a wind modifier added
/// * `w_boost` - value of wind modifier (which may or may not be added to p_base)
/// * `moisture` - modifier for tree moisture. It is subtracted from p_base
/// * `N` - number of rows in grid
/// * `M` - number of columns in grid
pub fn probability(p_base: f32, fire_idx: usize, tree_idx: usize, wind: &[i8; 2], w_boost: f32, moisture: f32, N: usize, M: usize) -> f32 {

    let tree_i: i8 = (tree_idx / M).try_into().unwrap();
    let tree_j: i8 = (tree_idx % M).try_into().unwrap();
    
    let is_wind: bool = (fire_idx as i8) == <usize as TryInto<i8>>::try_into(M).unwrap() * (tree_i + wind[0]) + tree_j + wind[1];
    
    if is_wind {
        return p_base + w_boost - moisture;
    }
    return p_base - moisture;

}

/// Changes state of the "world", aka the grid, by inspecting the cells with trees that are adjacent to cells with fire.
/// * `state` - the grid itself
/// * `directions` - representations of N, W, S, E directions
/// * `fire_indices` - indices of all cells currently on fire
/// * `p_base` - base probability without any modifiers
/// * `w_idx` - needed operations on i and j index to see whether there will be a wind modifier added
/// * `w_boost` - value of wind modifier (which may or may not be added to p_base)
/// * `moisture` - grid of modifiers for tree moisture. The modifier is subtracted from p_base
/// * `N` - number of rows in grid
/// * `M` - number of columns in grid
/// * `rng` - pseudorandom number generator
pub fn change_states(state: &mut Vec<Option<bool>>, directions: &HashMap<char, [i8; 2]>, fire_indices: &Vec<usize>, p_base: f32, w_idx: &[i8; 2], 
    w_boost: f32, moisture: &Vec<f32>, N: usize, M: usize, mut rng: &mut rand::prelude::ThreadRng) -> Vec<usize> {
    
    let mut trees_to_fire: Vec<usize> = vec![];
    for fire_idx in fire_indices {
        let fire_i: i8 = (*fire_idx / M).try_into().unwrap();
        let fire_j: i8 = (*fire_idx % M).try_into().unwrap();

        for c in ['N','W','S','E'] {
            let dir: [i8; 2] = directions[&c];

            let tree_i: usize = <i8 as TryInto<usize>>::try_into(fire_i + dir[0]).unwrap_or(fire_i as usize);
            let tree_j: usize = <i8 as TryInto<usize>>::try_into(fire_j + dir[1]).unwrap_or(fire_j as usize);

            let tree_idx: usize = M * tree_i + tree_j;
            let current: Option<bool>;
            match state.get(tree_idx) {
                Some(x) => { current = *x; }
                None => { continue; }
            }

            if current == Some(false) {
                let p_final: f32 = clamp::<f32>(probability(p_base, *fire_idx, tree_idx, w_idx, w_boost, moisture[tree_idx], N, M), 0 as f32, 1 as f32);
                let threshold: f32 = (rng.random_range(0..1001) as f32) / 1000 as f32;

                if p_final >= threshold && !trees_to_fire.contains(&tree_idx) {
                    trees_to_fire.push(tree_idx);
                }
            }
        }

        state[*fire_idx] = None;
        

    }

    for tree_idx in &trees_to_fire{
        state[*tree_idx] = Some(true);
    }

    return trees_to_fire;
}

/// Checks if there is fire present in grid
/// * `state` - the grid itself
pub fn check_fire(state: &Vec<Option<bool>>) -> bool {

    return state.contains(&Some(true));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deep_thought_test() {
        assert_eq!(42, 42);
    }
}
