use std::collections::HashSet;

fn redistribute(mem: &mut [i32]) {
    let size = mem.len();
    
}

fn check_loop_iterations(input: &mut [i32]) -> i32 {
    let mut iters = 0;
    let mut iter_map = HashSet::<Vec<i32>>::new();
    iters
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex_input() -> Vec<i32> {
        vec![0, 2, 7, 0]
    }

    fn input() -> Vec<i32> {
        let bytes = include_bytes!("../../../input.txt");
        let vec = String::from_utf8_lossy(bytes)
                         .split_whitespace()
                         .map(|c| c.parse::<i32>().unwrap())
                         .collect();
        vec
    }

    #[test]
    fn it_works() {
        let mut input = ex_input();
        let iters = check_loop_iterations(&mut input);
        assert_eq!(iters, 5);
    }
}
