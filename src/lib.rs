pub enum Card {
    Pip(u32),
    Face,
    Ace,
}
impl Card {
    fn value(&self) -> u32 {
        match *self {
            Card::Pip(v) => v,
            Card::Face => 10,
            Card::Ace => 11,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Score {
    Value(u32),
    Blackjack,
    Bust,
}

pub fn score(h: Vec<Card>) -> Score {
    score_from_sum_and_aces(h.iter().fold((0, false), |acc, x| add_to_sum_and_aces(acc, x)))
}

fn score_from_sum_and_aces(s_and_a: (u32, bool)) -> Score {
    hand_value_into_score(hand_value(s_and_a))
}

pub fn add_to_sum_and_aces(acc: (u32, bool), x: &Card) -> (u32, bool) {
    match x {
        Card::Ace => (acc.0 + 1, true),
        _ => (acc.0 + x.value(), acc.1),
    }
}

fn hand_value_into_score(sum: u32) -> Score {
    const BLACKJACK: u32 = 21;
    match sum {
        s if s > BLACKJACK => Score::Bust,
        s if s == BLACKJACK => Score::Blackjack,
        _ => Score::Value(sum),
    }
}

fn hand_value(sum_and_aces: (u32, bool)) -> u32 {
    aces_value(sum_and_aces) + sum_and_aces.0
}

fn aces_value(sum_and_aces: (u32, bool)) -> u32 {
    if sum_and_aces.1 && sum_and_aces.0 < 12 {
        return 10;
    }
    0
}
