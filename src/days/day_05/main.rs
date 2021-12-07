use std::collections::HashMap;
use crate::Part;
use crate::problem::{Problem, AOCResult};

#[allow(dead_code)]
pub fn day05_launch(part: Part) -> AOCResult<String> {
    let strings = parse_input()?;
    match part {
        Part::Part1 => part1(&strings),
        Part::Part2 => part2(&strings)
    }
}

fn part1(strings: &Vec<String>) -> AOCResult<String> {
    Ok(strings.iter().filter(|s| is_string_nice_part1(s)).count().to_string())
}

fn part2(strings: &Vec<String>) -> AOCResult<String> {
    Ok(strings.iter().filter(|s| is_string_nice_part2(s)).count().to_string())
}

fn is_string_nice_part1(string: &str) -> bool {
    let mut collector = CharCollector::default();
    string.chars().for_each(|c| collector.push_char(c));
    return collector.is_string_nice();
}

fn is_string_nice_part2(strings: &str) -> bool {
    let chars = strings.chars().collect();
    return is_string_nice_part2_criteria_two(&chars) && is_string_nice_part2_criteria_one(&chars);
}

fn is_string_nice_part2_criteria_one(chars: &Vec<char>) -> bool {
    let mut pair_positions:HashMap<(char,char), usize> = HashMap::new();

    for idx in 0..chars.len() - 1 {
        let c0 = chars.get(idx).unwrap();
        let c1 = chars.get(idx + 1).unwrap();

        let pair = (*c0,*c1);

        let existing = pair_positions.get(&pair);
        if let Some(previous_position) = existing {
            if *previous_position+2 <= idx {
                return true;
            }
        }
        else {
            pair_positions.insert(pair,idx);
        }

    };
    return false;

}

fn is_string_nice_part2_criteria_two(chars: &Vec<char>) -> bool {
    for idx in 0..chars.len()-2 {
        let c0 = chars.get(idx).unwrap();
        let c2 = chars.get(idx + 2).unwrap();
        if c0 == c2 {
            return true;
        }
    }
    return false;
}


struct CharCollector {
    first: bool,
    previous: char,
    nb_vowels: usize,
    has_double: bool,
    has_naughty_pair: bool,
}

impl Default for CharCollector {
    fn default() -> Self {
        CharCollector { first: true, previous: 'a', nb_vowels: 0, has_double: false, has_naughty_pair: false }
    }
}

impl CharCollector {
    fn is_string_nice(&self) -> bool {
        return !self.has_naughty_pair && self.nb_vowels >= 3 && self.has_double;
    }

    fn push_char(&mut self, c: char) {
        if is_vowel(c) {
            self.nb_vowels += 1;
        }

        if self.first {
            self.first = false;
        } else {
            self.has_naughty_pair |= is_naughty_pair(self.previous, c);
            self.has_double |= self.previous == c;
        }
        self.previous = c;
    }
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn is_naughty_pair(c1: char, c2: char) -> bool {
    //    ab, cd, pq, or xy
    match (c1, c2) {
        ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => true,
        _ => false
    }
}


#[allow(dead_code)]
fn parse_input() -> AOCResult<Vec<String>> {
    Problem::factory(false)(5).read_input_as_vec_of_lines()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_05::main::{is_string_nice_part1, is_string_nice_part2};

    #[test]
    fn day05_part1_test_01() {
        let result = is_string_nice_part1("ugknbfddgicrmopn");
        assert_eq!(true, result)
    }

    #[test]
    fn day05_part1_test_02() {
        let result = is_string_nice_part1("aaa");
        assert_eq!(true, result)
    }

    #[test]
    fn day05_part1_test_03() {
        let result = is_string_nice_part1("jchzalrnumimnmhp");
        assert_eq!(false, result)
    }

    #[test]
    fn day05_part1_test_04() {
        let result = is_string_nice_part1("haegwjzuvuyypxyu");
        assert_eq!(false, result)
    }

    #[test]
    fn day05_part1_test_05() {
        let result = is_string_nice_part1("dvszwmarrgswjxmb");
        assert_eq!(false, result)
    }


    #[test]
    fn day05_part2_test_01() {
        let result = is_string_nice_part2("qjhvhtzxzqqjkmpb");
        assert_eq!(true, result)
    }

    #[test]
    fn day05_part2_test_02() {
        let result = is_string_nice_part2("xxyxx");
        assert_eq!(true, result)
    }

    #[test]
    fn day05_part2_test_03() {
        let result = is_string_nice_part2("uurcxstgmygtbstg");
        assert_eq!(false, result)
    }

    #[test]
    fn day05_part2_test_04() {
        let result = is_string_nice_part2("ieodomkazucvgmuy");
        assert_eq!(false, result)
    }
}