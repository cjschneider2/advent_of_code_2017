use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y}
    }

    /// The manhattan distance to (0, 0)
    pub fn manhattan_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn get_coord_for_step(n: i32) -> Point {
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut d = 1;
    let mut m = 1;

    let mut idx = 0;
    'main: loop {
        // side up/down
        while 2 * x * d < m {
            idx += 1;
            if idx >= n { break 'main; }
            x = x + d;
        }
        // side left/right
        while 2 * y * d < m {
            idx += 1;
            if idx >= n { break 'main; }
            y = y + d;
        }
        d = -d;
        m += 1;
    }

    Point::new(x, y)
}

pub fn sprial_steps_part_1 (n: i32) -> i32 {
    let point = get_coord_for_step(n);
    point.manhattan_dist()
}

pub fn sprial_steps_part_2 (n: i32) -> u32 {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut val = 0;
    println!("\nmax_steps:{}", n);
    if n == 1 { return 2; }
    map.insert((0, 0), 1);
    for idx in 2..n+2 {
        // get coord of current step
        let p = get_coord_for_step(idx);
        // get values of neighboring steps
        let mut _v = 0;
        if let Some(ul) = map.get(&( p.x + 1, p.y + 1)) { println!("ul: {}", ul); _v += ul; }
        if let Some(uc) = map.get(&( p.x + 0, p.y + 1)) { println!("uc: {}", uc); _v += uc; }
        if let Some(ur) = map.get(&( p.x - 1, p.y + 1)) { println!("ur: {}", ur); _v += ur; }
        if let Some(cl) = map.get(&( p.x + 1, p.y + 0)) { println!("cl: {}", cl); _v += cl; }
        if let Some(cr) = map.get(&( p.x - 1, p.y + 0)) { println!("cr: {}", cr); _v += cr; }
        if let Some(ll) = map.get(&( p.x + 1, p.y - 1)) { println!("ll: {}", ll); _v += ll; }
        if let Some(lc) = map.get(&( p.x + 0, p.y - 1)) { println!("lc: {}", lc); _v += lc; }
        if let Some(lr) = map.get(&( p.x - 1, p.y - 1)) { println!("lr: {}", lr); _v += lr; }
        map.insert((p.x, p.y), _v);
        val = _v;
        println!("step: {}, coord: {:?}, val:{}\n", idx, p, val);
        if val > n as u32 { break; }
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> i32 { 361527 }
    
    #[test]
    fn ex_sq_1() {
        let square = 1;
        let res = sprial_steps_part_1(square);
        assert_eq!(res, 0);
    }
    
    #[test]
    fn ex_sq_12() {
        let square = 12;
        let res = sprial_steps_part_1(square);
        assert_eq!(res, 3);
    }
    
    #[test]
    fn ex_sq_23() {
        let square = 23;
        let res = sprial_steps_part_1(square);
        assert_eq!(res, 2);
    }
    
    #[test]
    fn ex_sq_1024() {
        let square = 1024;
        let res = sprial_steps_part_1(square);
        assert_eq!(res, 31);
    }

    #[test]
    fn test_part_1() {
        let square = input();
        let res = sprial_steps_part_1(square);
        assert_eq!(res, 326);
    }

    #[test]
    fn ex_1_part_2() {
        let square = 1;
        let res = sprial_steps_part_2(square);
        assert_eq!(res, 2);
    }

    #[test]
    fn ex_2_part_2() {
        let square = 2;
        let res = sprial_steps_part_2(square);
        assert_eq!(res, 2);
    }

    #[test]
    fn ex_3_part_2() {
        let square = 3;
        let res = sprial_steps_part_2(square);
        assert_eq!(res, 4);
    }

    #[test]
    fn ex_4_part_2() {
        let square = 4;
        let res = sprial_steps_part_2(square);
        assert_eq!(res, 5);
    }

    #[test]
    fn ex_5_part_2() {
        let square = 5;
        let res = sprial_steps_part_2(square);
        assert_eq!(res, 10);
    }

    #[test]
    fn ex_13_part_2() {
        let square = 13;
        let res = sprial_steps_part_2(square);
        assert_eq!(res, 23);
    }

    #[test]
    fn test_part_2() {
        let square = input();
        let res = sprial_steps_part_2(square);
        assert_eq!(res, 363010);
    }
}
