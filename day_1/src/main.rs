fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Most Calories: {}", most_calories_b(input))
}

fn most_calories(input: &str) -> u32 {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut calories = vec![];
    for elve in elves.iter() {
        let cals = elve.split("\n").map(|x| x.parse::<u32>().unwrap());
        let mut sum = 0;
        for cal in cals {
            sum += cal
        }
        calories.push(sum)
    }
    let mut largest = 0;
    for cal in calories.iter() {
        if *cal > largest {
            largest = *cal
        }
    }
    largest
}

fn most_calories_b(input: &str) -> u32 {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut calories = vec![];
    for elve in elves.iter() {
        let cals = elve.split("\n").map(|x| x.parse::<u32>().unwrap());
        let mut sum = 0;
        for cal in cals {
            sum += cal
        }
        calories.push(sum)
    }
    calories.sort();
    calories[calories.len() - 1] + calories[calories.len() - 2] + calories[calories.len() - 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_calories() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(most_calories(input), 24000)
    }

    #[test]
    fn test_most_calories_b() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(most_calories_b(input), 45000)
    }
}
