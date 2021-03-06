pub fn row_difference(input: &str) -> u32 /* row difference */ {
    let digits: Vec<u32> = input.split_whitespace()
                                .map(|s| s.parse::<u32>().unwrap())
                                .collect();
    let mut max = 0;
    let mut min = std::u32::MAX;
    for digit in digits {
        if digit < min { min = digit; }
        if digit > max { max = digit; }
    }
    max - min
}

pub fn checksum_matrix(input: &str) -> u32 /* matrix checksum */ {
    let checksum: u32 = input.split("\n")
                             .fold(0, |acc, row| {
                                 acc + row_difference(row)
                             });
    checksum
}

pub fn row_division(input: &str) -> u32 /* row div */ {
    let digits: Vec<u32> = input.split_whitespace()
                                .map(|s| s.parse::<u32>().unwrap())
                                .collect();
    for (idx, digit) in digits.iter().enumerate() {
        for (idy, d) in digits.iter().enumerate() {
            if idx == idy { continue }
            if digit % d == 0 {
                return digit / d
            }
        }
    }
    0
}

pub fn matrix_division_sum(input: &str) -> u32 /* matrix div sum */ {
    let checksum: u32 = input.split("\n")
                             .fold(0, |acc, row| {
                                 acc + row_division(row)
                             });
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
r#"4347 3350 196  162  233  4932 4419 3485 4509 4287 4433 4033 207  3682 2193 4223
648  94   778  957  1634 2885 1964 2929 2754 89   972  112  80   2819 543  2820
400  133  1010 918  1154 1008 126  150  1118 117  148  463  141  940  1101 89
596  527  224  382  511  565  284  121  643  139  625  335  657  134  125  152
2069 1183 233  213  2192 193  2222 2130 2073 2262 1969 2159 2149 410  181  1924
1610 128  1021 511  740  1384 459  224  183  266  152  1845 1423 230  1500 1381
5454 3936 250  5125 244  720  5059 202  4877 5186 313  6125 172  727  1982 748
3390 3440 220  228  195  4525 1759 1865 1483 5174 4897 4511 5663 4976 3850 199
130  1733 238  1123 231  1347 241  291  1389 1392 269  1687 1359 1694 1629 1750
1590 1394 101  434  1196 623  1033 78   890  1413 74   1274 1512 1043 1103 84
203  236  3001 1664 195  4616 2466 4875 986  1681 152  3788 541  4447 4063 5366
216  4134 255  235  1894 5454 1529 4735 4962 220  2011 2475 185  5060 4676 4089
224  253  19   546  1134 3666 3532 207  210  3947 2290 3573 3808 1494 4308 4372
134  130  2236 118  142  2350 3007 2495 2813 2833 2576 2704 169  2666 2267 850
401  151  309  961  124  1027 1084 389  1150 166  1057 137  932  669  590  188
784  232  363  316  336  666  711  430  192  867  628  57   222  575  622  234"#
    }

    /*
    part 1
    */

    fn ex_input_part_1() -> &'static str {
        r#"5 1 9 5
7 5 3
2 4 6 8"#
    }

    #[test]
    fn ex_row_diff_1() {
        let input = "5 1 9 5";
        let res = row_difference(input);
        assert_eq!(res, 8);
    }

    #[test]
    fn ex_row_diff_2() {
        let input = "7 5 3";
        let res = row_difference(input);
        assert_eq!(res, 4);
    }

    #[test]
    fn ex_row_diff_3() {
        let input = "2 4 6 8";
        let res = row_difference(input);
        assert_eq!(res, 6);
    }

    #[test]
    fn ex_matrix_part_1() {
        let input = ex_input_part_1();
        let res = checksum_matrix(input);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_matrix_part_1() {
        let input = test_input();
        let res = checksum_matrix(input);
        assert_eq!(res, 47136);
    }

    /*
    part 2
    */

    fn ex_input_part_2() -> &'static str {
        r#"5 9 2 8
9 4 7 3
3 8 6 5"#
    }

    #[test]
    fn ex_row_div_1() {
        let input = "5 9 2 8";
        let res = row_division(input);
        assert_eq!(res, 4);
    }

    #[test]
    fn ex_row_div_2() {
        let input = "9 4 7 3";
        let res = row_division(input);
        assert_eq!(res, 3);
    }

    #[test]
    fn ex_row_div_3() {
        let input = "3 8 6 5";
        let res = row_division(input);
        assert_eq!(res, 2);
    }

    #[test]
    fn ex_matrix_part_2() {
        let input = ex_input_part_2();
        let res = matrix_division_sum(input);
        assert_eq!(res, 9);
    }

    #[test]
    fn test_matrix_part_2() {
        let input = test_input();
        let res = matrix_division_sum(input);
        assert_eq!(res, 250);
    }
}
