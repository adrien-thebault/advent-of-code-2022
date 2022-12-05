use std::{cmp::Ordering, io::ErrorKind};

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("input.txt"))
        .lines()
        .map(|l| {
            let mut split = l
                .split(' ')
                .map(Game::try_from)
                .map(|r| r.expect("input contains invalid data"));

            (
                split.next().expect("not enough data"), // opponent
                split.next().expect("not enough data"), // me
            )
        })
        .collect::<Vec<_>>();

    // Exo 1
    println!(
        "{}",
        input
            .iter()
            .map(|(opponent, me)| { *me as usize + Outcome::from((opponent, me)) as usize })
            .sum::<usize>()
    );

    // Exo 2
    println!(
        "{}",
        input
            .iter()
            .map(|(opponent, me)| {
                let expected = Outcome::from(me);

                expected as usize
                    + *(if expected == Outcome::Draw {
                        opponent
                    } else if expected == Outcome::Loss {
                        match *opponent {
                            Game::Paper => &Game::Rock,
                            Game::Rock => &Game::Scissors,
                            Game::Scissors => &Game::Paper,
                        }
                    } else {
                        match *opponent {
                            Game::Paper => &Game::Scissors,
                            Game::Rock => &Game::Paper,
                            Game::Scissors => &Game::Rock,
                        }
                    }) as usize
            })
            .sum::<usize>()
    )
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Game {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if (self == &Game::Rock && other == &Game::Paper)
            || (self == &Game::Paper && other == &Game::Scissors)
            || (self == &Game::Scissors && other == &Game::Rock)
        {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl TryFrom<&str> for Game {
    type Error = ErrorKind;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Game::Rock),
            "B" | "Y" => Ok(Game::Paper),
            "C" | "Z" => Ok(Game::Scissors),
            _ => Err(ErrorKind::InvalidData),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<(&Game, &Game)> for Outcome {
    fn from((opponent, me): (&Game, &Game)) -> Self {
        let cmp = me.partial_cmp(opponent).expect("could not compare");
        if cmp == Ordering::Equal {
            Outcome::Draw
        } else if cmp == Ordering::Less {
            Outcome::Loss
        } else {
            Outcome::Win
        }
    }
}

impl From<&Game> for Outcome {
    fn from(value: &Game) -> Self {
        match value {
            Game::Rock => Outcome::Loss,
            Game::Paper => Outcome::Draw,
            Game::Scissors => Outcome::Win,
        }
    }
}
