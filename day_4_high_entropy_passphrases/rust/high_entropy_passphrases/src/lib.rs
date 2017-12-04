use std::collections::HashSet;

fn is_anagram(word_1: &str, word_2: &str) -> bool {
    if word_1.len() != word_2.len() {
        return false
    }

    let mut _1: Vec<char> = word_1.chars().collect();
    let mut _2: Vec<char> = word_2.chars().collect();

    _1.sort_unstable();
    _2.sort_unstable();

    for (idx, c) in _1.iter().enumerate() {
        if *c != _2[idx] { return false; }
    }

    true
}

fn is_valid_passphrase(input: &str, check_anagrams: bool) -> bool {
    let mut set: HashSet<String> = HashSet::new();
    let mut word_count = 0;
    if input.len() == 0 { return false }
    for substr in input.split_whitespace() {
        if check_anagrams {
            for word in set.iter() {
                if is_anagram(word, substr) {
                    return false;
                }
            }
        }
        set.insert(substr.into());
        word_count += 1;
    }
    word_count == set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        let bytes = include_bytes!("../../../input.txt");
        String::from_utf8_lossy(bytes).into()
    }

    #[test]
    fn ex_1_part_1() {
        let input = "aa bb cc dd";
        assert_eq!(is_valid_passphrase(input, false), true);
    }

    #[test]
    fn ex_2_part_1() {
        let input = "aa bb cc dd aa";
        assert_eq!(is_valid_passphrase(input, false), false);
    }

    #[test]
    fn ex_3_part_1() {
        let input = "aa bb cc dd aaa";
        assert_eq!(is_valid_passphrase(input, false), true);
    }

    #[test]
    fn ex_1_part_2() {
        let input = "abcde fghij";
        assert_eq!(is_valid_passphrase(input, false), true);
    }

    #[test]
    fn ex_2_part_2() {
        let input = "abcde xyz ecdab";
        assert_eq!(is_valid_passphrase(input, false), false);
    }

    #[test]
    fn ex_3_part_2() {
        let input = "a ab abc abd abf abj";
        assert_eq!(is_valid_passphrase(input, false), true);
    }

    #[test]
    fn ex_4_part_2() {
        let input = "iiii oiii ooii oooi oooo";
        assert_eq!(is_valid_passphrase(input, false), true);
    }

    #[test]
    fn ex_5_part_2() {
        let input = "oiii ioii iioi iiio";
        println!("{}", pw);
        assert_eq!(is_valid_passphrase(input, false), false);
    }

    #[test]
    fn test_input_part_1() {
        println!("");
        let pw_set = input();
        let mut valid_pw_count = 0;
        let mut count = 0;
        for pw in pw_set.split("\n") {
            if is_valid_passphrase(pw, false) {
                valid_pw_count += 1;
                println!("valid  : {}", pw);
            } else {
                println!("invalid: {}", pw);
            }
            count += 1;
        }
        println!("valid: {}", valid_pw_count);
        println!("count: {}", count);
        assert_eq!(valid_pw_count, 386);
    }

    // #[test]
    fn test_input_part_2() {
        println!("");
        let pw_set = input();
        let mut valid_pw_count = 0;
        let mut count = 0;
        for pw in pw_set.split("\n") {
            if is_valid_passphrase(pw, true) {
                valid_pw_count += 1;
                println!("valid  : {}", pw);
            } else {
                println!("invalid: {}", pw);
            }
            count += 1;
        }
        println!("valid: {}", valid_pw_count);
        println!("count: {}", count);
        assert_eq!(valid_pw_count, 0);
    }
}
