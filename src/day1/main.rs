use std::{cmp::Reverse, str::FromStr};

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("input.txt"));

    let groups = input.split("\n\n");
    let mut groups_sum = groups
        .into_iter()
        .map(|g| {
            g.split('\n')
                .into_iter()
                .map(usize::from_str)
                .map(|r| r.expect("invalid usize"))
                .sum()
        })
        .collect::<Vec<usize>>();
    groups_sum.sort_by_key(|x| Reverse(*x));

    println!("{}", groups_sum.iter().max().expect("no max found"));
    println!("{}", groups_sum.iter().take(3).sum::<usize>());
}
