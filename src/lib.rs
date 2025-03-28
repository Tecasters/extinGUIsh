pub fn show_output(state: Vec<Option<bool>>, N: usize, M: usize){

    let mut output: char;
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deep_thought_test() {
        assert_eq!(42, 42);
    }
}
