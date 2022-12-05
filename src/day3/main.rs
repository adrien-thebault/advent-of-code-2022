use std::collections::HashMap;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("input.txt"));

    let mut priorities = HashMap::new();
    priorities.extend(('a'..='z').map(|c| (c, c as usize - 96)));
    priorities.extend(('A'..='Z').map(|c| (c, c as usize - 64 + 26)));

    // Part 1
    println!(
        "{}",
        input
            .lines()
            .map(|l| {
                let (left, right) = l.split_at((l.len() / 2) as usize);
                left.chars()
                    .find(|c| right.contains(*c))
                    .expect("no common char found")
            })
            .map(|c| priorities.get(&c).expect("priority not found"))
            .sum::<usize>()
    );

    // Part 2
    println!(
        "{}",
        input
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|elves| {
                let (first, second, third) = (elves[0], elves[1], elves[2]);
                first
                    .chars()
                    .find(|c| second.contains(*c) && third.contains(*c))
                    .expect("no common char found")
            })
            .map(|c| priorities.get(&c).expect("priority not found"))
            .sum::<usize>()
    )
}
