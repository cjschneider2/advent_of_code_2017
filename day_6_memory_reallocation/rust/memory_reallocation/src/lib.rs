use std::collections::HashMap;

fn redistribute(mem: &mut [i32], index: usize) {
    let size = mem.len();
    let mut blocks = mem[index];
    let mut idx = index + 1; 
    mem[index] = 0;
    while blocks > 0 {
        if idx == size {
            idx = 0;
        }
        mem[idx] += 1;
        blocks -= 1;
        idx += 1;
    } 
}

fn most_blocks(mem: &[i32]) -> usize {
    let mut index = 0;
    let mut value = 0;
    for (idx, blk) in mem.iter().enumerate() {
        if *blk > value {
            index = idx;
            value = *blk;
        }
    }
    index
}

fn check_loop_iterations(mem: &mut [i32]) -> (i32, i32) {
    let mut iters = 0;
    let mut iter_map = HashMap::<Vec<i32>, i32>::new();
    iter_map.insert(mem.into(), iters);
    loop {
        let blk = most_blocks(mem);
        redistribute(mem, blk);
        iters += 1;
        let vec: Vec<i32> = mem.into();
        let c_k = iter_map.contains_key(&vec);
        if c_k {
            let k = iter_map.get(&vec).unwrap();
            return (iters, iters - *k);
        } else {
            iter_map.insert(vec, iters);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex_input() -> Vec<i32> {
        vec![0, 2, 7, 0]
    }

    fn input() -> Vec<i32> {
        let bytes = include_str!("../../../input.txt");
        let vec = bytes.split_whitespace()
                       .map(|c| c.parse::<i32>().unwrap())
                       .collect();
        vec
    }

    #[test]
    fn test_ex_input() {
        let mut input = ex_input();
        let (iters, _) = check_loop_iterations(&mut input);
        assert_eq!(iters, 5);
    }

    #[test]
    fn test_input_p1() {
        let mut input = input();
        let (iters, _)= check_loop_iterations(&mut input);
        assert_eq!(iters, 14029);
    }

    #[test]
    fn test_input_p2() {
        let mut input = input();
        let (_, iters)= check_loop_iterations(&mut input);
        assert_eq!(iters, 2765);
    }
}
