#[cfg(test)]
use demonstrate::demonstrate;

#[cfg(test)]
use blackjack::Card;

#[cfg(test)]
fn char_to_card(c: &char) -> Card {
    match *c {
        '2' => Card::Pip(2),
        '3' => Card::Pip(3),
        '4' => Card::Pip(4),
        '5' => Card::Pip(5),
        '6' => Card::Pip(6),
        '7' => Card::Pip(7),
        '8' => Card::Pip(8),
        '9' => Card::Pip(9),
        'T' => Card::Pip(10),
        'J' => Card::Face,
        'Q' => Card::Face,
        'K' => Card::Face,
        'A' => Card::Ace,
        _ => unimplemented!(),
    }
}
#[cfg(test)]
fn make_hand(input: Vec<char>) -> Vec<Card> {
    input.iter()
        .map(|c| char_to_card(c))
        .collect::<Vec<Card>>()
}

#[cfg(test)]
demonstrate! {
    describe "In a new context" {
        use super::*;
        use blackjack::*;
        use hamcrest2::prelude::*;

        it "empty hand scores zero" {
            let hand: Vec<Card> = Vec::new();
            assert_that!(score(hand), eq(Score::Value(0)));
        }
        it "one deuce scores two" {
            assert_that!(score(make_hand(vec!['2'])), eq(Score::Value(2)));
        }
        it "one pip cards scores its pip value" {
            assert_that!(score(make_hand(vec!['3'])), eq(Score::Value(3)));
            assert_that!(score(make_hand(vec!['4'])), eq(Score::Value(4)));
            assert_that!(score(make_hand(vec!['5'])), eq(Score::Value(5)));
            assert_that!(score(make_hand(vec!['6'])), eq(Score::Value(6)));
            assert_that!(score(make_hand(vec!['7'])), eq(Score::Value(7)));
            assert_that!(score(make_hand(vec!['8'])), eq(Score::Value(8)));
            assert_that!(score(make_hand(vec!['9'])), eq(Score::Value(9)));
            assert_that!(score(make_hand(vec!['T'])), eq(Score::Value(10)));
        }
        it "one face card scores ten" {
            assert_that!(score(make_hand(vec!['J'])), eq(Score::Value(10)));
            assert_that!(score(make_hand(vec!['Q'])), eq(Score::Value(10)));
            assert_that!(score(make_hand(vec!['K'])), eq(Score::Value(10)));
        }
        it "one ace scores eleven" {
            assert_that!(score(make_hand(vec!['A'])), eq(Score::Value(11)));
        }
        it "two pips cards scores their sum" {
            assert_that!(score(make_hand(vec!['3', '2'])), eq(Score::Value(5)));
            assert_that!(score(make_hand(vec!['T', 'T'])), eq(Score::Value(20)));
        }
        it "one pips card and one face card scores ten plus pips" {
            assert_that!(score(make_hand(vec!['K', '2'])), eq(Score::Value(12)));
            assert_that!(score(make_hand(vec!['T', 'J'])), eq(Score::Value(20)));
        }
        it "two face cards scores twenty" {
            assert_that!(score(make_hand(vec!['K', 'K'])), eq(Score::Value(20)));
            assert_that!(score(make_hand(vec!['J', 'Q'])), eq(Score::Value(20)));
        }
        it "several pips cards with sum 20 or less scores their sum" {
            assert_that!(score(make_hand(vec!['2', '2', '2', '2', '2', '2', '2', '2', '2', '2'])), eq(Score::Value(20)));
        }
        it "one ace and one face card scores blackjack" {
            assert_that!(score(make_hand(vec!['K', 'A'])), eq(Score::Blackjack));
        }
        it "several pips cards with sum 21 scores blackjack" {
            assert_that!(score(make_hand(vec!['2', '2', '2', '2', '2', '2', '2', '2', '2', '3'])), eq(Score::Blackjack));
        }
        it "face card plus several pips cards with sum 21 scores blackjack" {
            assert_that!(score(make_hand(vec!['2', '2', '2', '2', 'Q', '3'])), eq(Score::Blackjack));
        }
        it "several pips cards with sum greater than 21 scores bust" {
            assert_that!(score(make_hand(vec!['2', '2', '2', '2', '2', '2', '2', '2', '2', '4'])), eq(Score::Bust));
        }
        it "three face cards scores bust" {
            assert_that!(score(make_hand(vec!['J', 'Q', 'K'])), eq(Score::Bust));
        }
        it "three aces scores thirteen" {
            assert_that!(score(make_hand(vec!['A', 'A', 'A'])), eq(Score::Value(13)));
        }
        it "eleven aces scores blackjack" {
            assert_that!(score(make_hand(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'])), eq(Score::Blackjack));
        }
        it "twelve aces scores twelve" {
            assert_that!(score(make_hand(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'])), eq(Score::Value(12)));
        }
        it "twenty one aces scores blackjack" {
            assert_that!(score(make_hand(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'])), eq(Score::Blackjack));
        }
        it "twenty two aces scores bust" {
            assert_that!(score(make_hand(vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'])), eq(Score::Bust));
        }
    }
}
