const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const LOST: u32 = 0;
const DRAW: u32 = 3;
const WON: u32 = 6;

pub fn process_part_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut hands = l.split(" ");
            let other = hands.next().unwrap();
            let my = hands.next().unwrap();
            let mut points = match my {
                "X" => ROCK,
                "Y" => PAPER,
                "Z" => SCISSORS,
                _ => panic!(),
            };

            if my == "X" && other == "A" || my == "Y" && other == "B" || my == "Z" && other == "C" {
                points += DRAW
            } else if my == "X" && other == "B"
                || my == "Y" && other == "C"
                || my == "Z" && other == "A"
            {
                points += LOST
            } else {
                points += WON
            }
            points
        })
        .sum()
}

pub fn process_part_2(input: &str) -> u32 {
    let processed: String = input
        .trim()
        .lines()
        .map(|l| {
            let mut hands = l.split(" ");
            let other = hands.next().unwrap();
            let command = hands.next().unwrap();
            match command {
                "X" => match other {
                    "A" => "A Z\n",
                    "B" => "B X\n",
                    "C" => "C Y\n",
                    _ => panic!(),
                },
                "Y" => match other {
                    "A" => "A X\n",
                    "B" => "B Y\n",
                    "C" => "C Z\n",
                    _ => panic!(),
                },
                "Z" => match other {
                    "A" => "A Y\n",
                    "B" => "B Z\n",
                    "C" => "C X\n",
                    _ => panic!(),
                },
                _ => panic!(),
            }
        })
        .collect();
    process_part_1(&processed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(process_part_1(input), 15);
    }

    #[test]
    fn part_2() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(process_part_2(input), 12);
    }

}
