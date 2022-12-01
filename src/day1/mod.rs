use std::collections::HashMap;

pub fn part1() {
    let mut elf_index = 0;
    let mut elf_calories: HashMap<i64, Vec<i64>> = HashMap::new();
    elf_calories.insert(elf_index, Vec::new());

    let input = super::files::read_lines("src/day1/input.txt");

    // Could do this in one loop but the next one probably requires
    // some other transformation so I'm separating them
    for e in input.iter() {
        if e == "" {
            // New elf new vec
            elf_index += 1;
            elf_calories.insert(elf_index, Vec::new());

            continue;
        }

        let calories = e.parse::<i64>().unwrap();
        elf_calories.get_mut(&elf_index).unwrap().push(calories);
    }

    let sums = elf_calories
        .iter()
        .map(|(_elf, balances)| balances.iter().sum::<i64>());

    println!("Part1 Max: {:?}", sums.into_iter().max());
}

pub fn part2() {
    let mut elf_index = 0;
    let mut elf_calories: HashMap<i64, Vec<i64>> = HashMap::new();
    elf_calories.insert(elf_index, Vec::new());

    let input = super::files::read_lines("src/day1/input.txt");

    for e in input.iter() {
        if e == "" {
            // New elf new vec
            elf_index += 1;
            elf_calories.insert(elf_index, Vec::new());

            continue;
        }

        let calories = e.parse::<i64>().unwrap();
        elf_calories.get_mut(&elf_index).unwrap().push(calories);
    }

    let mut sums = elf_calories
        .iter()
        .map(|(_elf, balances)| balances.iter().sum::<i64>())
        .collect::<Vec<i64>>();
    sums.sort();

    let sum: i64 = sums.iter().rev().take(3).sum();

    println!("Part2 Max: {}", sum);
}
