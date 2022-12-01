use std::collections::HashMap;

pub fn part1() {
    let mut elf_index: i64 = 0;
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

    let mut max = 0;
    for (_elf, balances) in elf_calories.iter() {
        let sum: i64 = balances.iter().sum();
        if sum > max {
            max = sum;
        }
    }

    println!("Part1 Max: {}", max);
}

pub fn part2() {
    let mut elf_index: i64 = 0;
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

    let mut sums = Vec::new();
    for (_elf, balances) in elf_calories.iter() {
        let sum: i64 = balances.iter().sum();
        sums.push(sum);
    }
    sums.sort();
    let sum: i64 = sums.iter().rev().take(3).sum();
    println!("Part1 Max: {}", sum);
}
