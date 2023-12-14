use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Knight,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn from_char(c: char, knight_as_joker: bool) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => if knight_as_joker { Card::Joker } else { Card::Knight },
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card char"),
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Card::Joker => 0,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Knight => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn value(&self) -> usize {
        match self {
            HandType::HighCard => 2,
            HandType::OnePair => 4,
            HandType::TwoPairs => 6,
            HandType::ThreeOfAKind => 8,
            HandType::FullHouse => 10,
            HandType::FourOfAKind => 12,
            HandType::FiveOfAKind => 14,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn from_str(s: &str, knight_as_joker: bool) -> Hand {
        let mut cards = [Card::Two; 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = Card::from_char(c, knight_as_joker);
        }
        Hand { cards }
    }

    pub fn to_hand_type(&self) -> HandType {
        let mut card_count = heapless::FnvIndexMap::<Card, usize, 32>::new();
        let mut joker_count = 0;

        for card in self.cards.iter() {
            if *card == Card::Joker {
                joker_count += 1;
                continue;
            }
            let count = card_count.get(card).unwrap_or(&0) + 1;
            card_count.insert(*card, count).unwrap();
        }

        let mut values = heapless::Vec::<usize, 5>::new();
        for (_, &count) in card_count.iter() {
            values.push(count).unwrap();
        }
        values.sort_unstable();

        let highest_count = if !values.is_empty() { values[values.len() - 1] } else { 0 };
        let second_highest_count = if values.len() >= 2 { values[values.len() - 2] } else { 0 };

        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        match highest_count {
            5 => HandType::FiveOfAKind,
            4 => if joker_count == 1 { HandType::FiveOfAKind } else { HandType::FourOfAKind },
            3 => match joker_count {
                0 => match second_highest_count {
                    2 => HandType::FullHouse,
                    1 => HandType::ThreeOfAKind,
                    _ => panic!("Invalid second highest count"),
                }
                1 => HandType::FourOfAKind,
                2 => HandType::FiveOfAKind,
                _ => panic!("Invalid joker count"),
            },
            2 => match joker_count {
                0 => match second_highest_count {
                    2 => HandType::TwoPairs,
                    1 => HandType::OnePair,
                    _ => panic!("Invalid second highest count"),
                }
                1 => match second_highest_count {
                    2 => HandType::FullHouse,
                    1 => HandType::ThreeOfAKind,
                    _ => panic!("Invalid second highest count"),
                }
                2 => HandType::FourOfAKind,
                3 => HandType::FiveOfAKind,
                _ => panic!("Invalid joker count"),
            },
            1 => match joker_count {
                0 => HandType::HighCard,
                1 => HandType::OnePair,
                2 => HandType::ThreeOfAKind,
                3 => HandType::FourOfAKind,
                4 => HandType::FiveOfAKind,
                _ => panic!("Invalid joker count"),
            },
            _ => panic!("Invalid highest count"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.to_hand_type();
        let other_type = other.to_hand_type();

        match self_type.value().cmp(&other_type.value()) {
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match self_card.cmp(other_card) {
                        Ordering::Equal => continue,
                        x => return x,
                    }
                }
                panic!();
            }
            x => x,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandAndBid {
    hand: Hand,
    bid: usize,
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut hand_and_bids = heapless::Vec::<HandAndBid, 1024>::new();

    for line in s.lines() {
        let mut line_split = line.split(' ');
        let mut hand1 = Hand::from_str(line_split.next().unwrap(), false);
        let mut bid = line_split.next().unwrap().parse::<usize>().unwrap();
        let mut hand_and_bid = HandAndBid { hand: hand1, bid };
        hand_and_bids.push(hand_and_bid).unwrap();
    }

    hand_and_bids.sort_unstable();

    let mut total = 0;

    for (rank, hand_and_bid) in hand_and_bids.iter().enumerate() {
        total += hand_and_bid.bid * (rank + 1);
    }

    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j7.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut hand_and_bids = heapless::Vec::<HandAndBid, 1024>::new();

    for line in s.lines() {
        let mut line_split = line.split(' ');
        let mut hand1 = Hand::from_str(line_split.next().unwrap(), true);
        let mut bid = line_split.next().unwrap().parse::<usize>().unwrap();
        let mut hand_and_bid = HandAndBid { hand: hand1, bid };
        hand_and_bids.push(hand_and_bid).unwrap();
    }

    hand_and_bids.sort_unstable();

    let mut total = 0;

    for (rank, hand_and_bid) in hand_and_bids.iter().enumerate() {
        total += hand_and_bid.bid * (rank + 1);
    }

    total
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j7.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j7_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(6440, _p1(include_str!("j7_test.txt")));
        assert_eq!(251029473, _p1(include_str!("j7.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(5905, _p2(include_str!("j7_test.txt")));
        assert_eq!(251003917, _p2(include_str!("j7.txt")));
    }
}