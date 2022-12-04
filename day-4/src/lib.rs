use std::ops::Range;

pub fn process_part_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let sections: Vec<Range<_>> = line
                .split(",")
                .map(|sections| {
                    let mut sections = sections.split("-");
                    sections.next().unwrap().parse::<u32>().unwrap()
                        ..sections.next().unwrap().parse::<u32>().unwrap()
                })
                .collect();
            let mut result = 0;
            if sections[0].start >= sections[1].start && sections[0].end <= sections[1].end
                || sections[1].start >= sections[0].start && sections[1].end <= sections[0].end
            {
                result += 1
            }
            result
        })
        .sum()
}

pub fn process_part_2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let sections: Vec<Range<_>> = line
                .split(",")
                .map(|sections| {
                    let mut sections = sections.split("-");
                    sections.next().unwrap().parse::<u32>().unwrap()
                        ..sections.next().unwrap().parse::<u32>().unwrap() + 1
                })
                .collect();
            let mut result = 0;
            for i in sections[0].clone().into_iter() {
                if sections[1].contains(&i) {
                    result += 1;
                    break;
                }
            }
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1() {
        assert_eq!(process_part_1(INPUT), 2)
    }

    #[test]
    fn part_2() {
        assert_eq!(process_part_2(INPUT), 4)
    }
}
