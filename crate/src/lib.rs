use holdem_hand_evaluator::{heads_up_win_rate, Hand, HandCategory, get_hand_category};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compute_win_rate(used_cards: Box<[i32]>) -> Box<[f64]> {
    let hand1 = Hand::new()
        .add_card(used_cards[0] as usize)
        .add_card(used_cards[1] as usize);
    let mut hand2 = Hand::new();
    for i in 2..4 {
        if used_cards[i] != -1 {
            hand2 = hand2.add_card(used_cards[i] as usize);
        }
    }
    let mut board = Hand::new();
    for i in 4..9 {
        if used_cards[i] != -1 {
            board = board.add_card(used_cards[i] as usize);
        }
    }
    let win_rate = heads_up_win_rate(&hand1, &hand2, &board, &Hand::new());
    Box::new([win_rate.0, win_rate.1])
}

#[wasm_bindgen]
pub fn analyze_range(input: Box<[i32]>) -> Box<[f64]> {
    let hand = Hand::new()
        .add_card(input[0] as usize)
        .add_card(input[1] as usize);
    let flop = Hand::new()
        .add_card(input[2] as usize)
        .add_card(input[3] as usize)
        .add_card(input[4] as usize);
    let dead = hand + flop;
    let mut range = vec![];
    for index in 0..(input[5] as usize) {
        let n = input[index + 6] as usize;
        let i = n / 13;
        let j = n % 13;
        if i == j {
            for s1 in 0..3usize {
                for s2 in s1+1..3 {
                    if (dead.mask & (i * 4 + s1) as u64) == 0 && (dead.mask & (i * 4 + s2) as u64 == 0) {
                        range.push(Hand::new().add_card(i * 4 + s1).add_card(i * 4 + s2));
                    }
                }
            }
        } else if i < j {
            // suited
            for s in 0..3 {
                if (dead.mask & (i * 4 + s) as u64) == 0 && (dead.mask & (j * 4 + s) as u64 == 0) {
                    range.push(Hand::new().add_card(i * 4 + s).add_card(j * 4 + s));
                }
            }
        } else {
            for s1 in 0..3 {
                for s2 in s1+1..3 {
                    if (dead.mask & (i * 4 + s1) as u64) == 0 && (dead.mask & (j * 4 + s2) as u64 == 0) {
                        range.push(Hand::new().add_card(i * 4 + s1).add_card(j * 4 + s2));
                    }
                    if (dead.mask & (i * 4 + s2) as u64) == 0 && (dead.mask & (j * 4 + s1) as u64 == 0) {
                        range.push(Hand::new().add_card(i * 4 + s2).add_card(j * 4 + s1));
                    }
                }
            }
        }
    }
    let num_range = range.len();
    println!("num_range: {}", num_range);
    let mut counts = vec![0; 9];
    for r in range {
        match get_hand_category((flop + r).evaluate()) {
            HandCategory::HighCard => counts[8] += 1,
            HandCategory::OnePair => counts[7] += 1,
            HandCategory::TwoPair => counts[6] += 1,
            HandCategory::ThreeOfAKind => counts[5] += 1,
            HandCategory::Straight => counts[4] += 1,
            HandCategory::Flush => counts[3] += 1,
            HandCategory::FullHouse => counts[2] += 1,
            HandCategory::FourOfAKind => counts[1] += 1,
            HandCategory::StraightFlush => counts[0] += 1,
        }
    }
    counts.into_iter().map(|c| c as f64 / num_range as f64).rev().collect::<Vec<_>>().into_boxed_slice()
}