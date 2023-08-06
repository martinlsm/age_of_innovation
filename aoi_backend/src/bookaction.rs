use itertools::Itertools;
use rand::seq::SliceRandom;

use crate::common::Books;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BookAction {
    cost: Books,
    effect: BookActionEffect,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BookActionEffect {
    GainPower,
    DiscStep,
    GainCoins,
    UpgradeToGuild,
    PointsPerGuild,
    Spades,
}

fn all_book_actions() -> Vec<BookAction> {
    vec![
        BookAction {
            cost: Books(1),
            effect: BookActionEffect::GainPower,
        },
        BookAction {
            cost: Books(1),
            effect: BookActionEffect::DiscStep,
        },
        BookAction {
            cost: Books(2),
            effect: BookActionEffect::GainCoins,
        },
        BookAction {
            cost: Books(2),
            effect: BookActionEffect::UpgradeToGuild,
        },
        BookAction {
            cost: Books(1),
            effect: BookActionEffect::PointsPerGuild,
        },
        BookAction {
            cost: Books(3),
            effect: BookActionEffect::Spades,
        },
    ]
}

pub fn new_game_random_book_actions() -> Vec<BookAction> {
    let mut rng = rand::thread_rng();
    let mut actions = all_book_actions();
    actions.shuffle(&mut rng);

    actions.into_iter().take(3).collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::helpers::contains_duplicates;

    use super::*;

    #[test]
    fn book_actions_no_duplicates() {
        for _ in 0..100 {
            // Run multiple times since randomness is involved.
            assert!(!contains_duplicates(&new_game_random_book_actions()))
        }
    }
}