use std::io;

struct Elf {
    calories: Vec<usize>,
}

fn main() {
    let elves = read_input_to_elves();

    println!(
        "Most calories carried by an elf: {}",
        find_most_calories_carried(&elves)
    );

    println!(
        "Total calories carried by top three Elves: {}",
        find_top_n_total_calories(&elves, 3)
    );
}

fn read_input_to_elves() -> Vec<Elf> {
    let lines = io::stdin().lines();
    lines.fold(Vec::new(), |mut elves, line| {
        let line = line.unwrap();
        if line.is_empty() {
            let elf = Elf {
                calories: Vec::new(),
            };
            elves.push(elf);
        } else {
            let elf = elves.iter_mut().last();
            match elf {
                Some(elf) => elf.calories.push(line.parse::<usize>().unwrap()),
                None => (),
            }
        }
        elves
    })
}

fn find_most_calories_carried(elves: &Vec<Elf>) -> usize {
    let max_calories = &elves
        .iter()
        .max_by(|x, y| {
            x.calories
                .iter()
                .sum::<usize>()
                .cmp(&y.calories.iter().sum::<usize>())
        })
        .unwrap()
        .calories;
    max_calories.iter().sum::<usize>()
}

fn find_top_n_total_calories(elves: &Vec<Elf>, n: usize) -> usize {
    let summed_calories = &mut elves
        .iter()
        .map(|elf| elf.calories.iter().sum())
        .collect::<Vec<usize>>();
    summed_calories.sort_by(|a, b| b.cmp(a));
    summed_calories.iter().take(n).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_most_calories_carried() {
        let elves = vec![
            Elf { calories: vec![1, 1] },
            Elf { calories: vec![1, 3, 5] },
            Elf { calories: vec![3, 4] }
        ];
        assert_eq!(9, find_most_calories_carried(&elves))
    }

    #[test]
    fn test_find_top_n_total_calories() {
        let elves = vec![
            Elf { calories: vec![1, 1] },
            Elf { calories: vec![1, 3, 5] },
            Elf { calories: vec![3, 4] },
            Elf { calories: vec![10, 5, 1]}
        ];
        assert_eq!(32, find_top_n_total_calories(&elves, 3))
    }
}