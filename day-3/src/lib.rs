const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn priority(item: char) -> u32 {
    (ALPHABET.find(item.to_lowercase().next().unwrap()).unwrap()
        + 1
        + if item.is_uppercase() { 26 } else { 0 }) as u32
}

pub fn process_part_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let backpack = line.split_at(line.len() / 2);
            let mut result = ' ';
            for item in backpack.0.chars() {
                if backpack.1.contains(item) {
                    result = item;
                    break;
                }
            }
            priority(result)
        })
        .sum()
}

pub fn process_part_2(input: &str) -> u32 {
    let backpacks: Vec<&str> = input.trim().lines().collect();
    let mut groups = vec![];
    for i in (0..backpacks.len()).step_by(3) {
        groups.push((backpacks[i], backpacks[i + 1], backpacks[i + 2]))
    }
    groups
        .iter()
        .map(|(b1, b2, b3)| {
            let mut result = ' ';
            for item in b1.chars() {
                if b2.contains(item) {
                    if b3.contains(item) {
                        result = item;
                        break;
                    }
                }
            }
            priority(result)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1() {
        assert_eq!(process_part_1(INPUT), 157);
    }

    #[test]
    fn part_2() {
        assert_eq!(process_part_2(INPUT), 70);
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('f'), 6);
    }
}
