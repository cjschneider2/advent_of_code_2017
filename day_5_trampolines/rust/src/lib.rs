
fn read_jump_tbl(input: &str) -> Vec<i32> {
    input.split_whitespace()
         .map(|i| i.parse::<i32>().unwrap())
         .collect::<Vec<i32>>()
}

fn do_jumps_part_1(input: &str) -> i32 {
    let mut jmp_tbl = read_jump_tbl(&input);
    let mut idx: i32 = 0;
    let mut jumps = 0;
    while let Some(val) = jmp_tbl.get_mut(idx as usize) {
        if idx + *val < 0 { break; }
        idx += *val;
        *val += 1;
        jumps += 1;
    }
    jumps
}

fn do_jumps_part_2(input: &str) -> i32 {
    let mut jmp_tbl = read_jump_tbl(&input);
    let mut idx: i32 = 0;
    let mut jumps = 0;
    while let Some(val) = jmp_tbl.get_mut(idx as usize) {
        if idx + *val < 0 { break; }
        idx += *val;
        if *val >= 3 {
            *val -= 1;
        } else {
            *val += 1;
        }
        jumps += 1;
    }
    jumps
}


#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        let bytes = include_bytes!("../../input.txt");
        String::from_utf8_lossy(bytes).into()
    }

    #[test]
    fn test_input_part_1() {
        let input = input();
        println!("starting: test_input");
        let jumps = do_jumps_part_1(&input);
        assert_eq!(jumps, 343467);
    }

    #[test]
    fn test_input_part_2() {
        let input = input();
        println!("starting: test_input");
        let jumps = do_jumps_part_2(&input);
        assert_eq!(jumps, 24774780);
    }

}
