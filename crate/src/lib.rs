use holdem_hand_evaluator::{heads_up_win_rate, Hand};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compute_win_rate(used_cards: Box<[i32]>) -> Box<[f64]> {
    let hand1 = Hand::new()
        .add_card(used_cards[0] as usize)
        .add_card(used_cards[1] as usize);
    let hand2 = Hand::new()
        .add_card(used_cards[2] as usize)
        .add_card(used_cards[3] as usize);
    let mut board = Hand::new();
    for i in 4..9 {
        if used_cards[i] != -1 {
            board = board.add_card(used_cards[i] as usize);
        }
    }
    let win_rate = heads_up_win_rate(&hand1, &hand2, &board, &Hand::new());
    Box::new([win_rate.0, win_rate.1])
}
