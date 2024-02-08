use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use std::collections::HashSet;

fn main() {
    let mut card = generate_bingo_card();
    println!("Your BINGO card: {:?}", card);

    let mut drawn_numbers = HashSet::new();
    let mut rng = thread_rng();

    while !is_winner(&card, &drawn_numbers) {
        let draw = rng.gen_range(1..=75); // Corrected line
        drawn_numbers.insert(draw);
        println!("Number drawn: {}", draw);
        if is_winner(&card, &drawn_numbers) {
            println!("BINGO! You've won!");
            break;
        }
    }
}

fn generate_bingo_card() -> Vec<HashSet<u32>> {
    let mut rng = thread_rng();
    let mut card = Vec::new();

    for _ in 0..5 {
        let mut column = HashSet::new();
        while column.len() < 5 {
            let number = rng.gen_range(1..=75); // This should work with rand 0.8.4 and newer
            column.insert(number);
        }
        card.push(column);
    }

    card
}

fn is_winner(card: &Vec<HashSet<u32>>, drawn_numbers: &HashSet<u32>) -> bool {
    // Check rows
    for row in card {
        if row.is_subset(drawn_numbers) {
            return true;
        }
    }

    // Check columns
    for i in 0..5 {
        let mut column = HashSet::new();
        for row in card {
            if let Some(&number) = row.iter().nth(i) {
                column.insert(number);
            }
        }
        if column.is_subset(drawn_numbers) {
            return true;
        }
    }

    false
}
