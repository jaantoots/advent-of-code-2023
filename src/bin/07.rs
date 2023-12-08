use std::io;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [char; 5],
    bid: u64,
}

impl Hand {
    fn groups_triplets(&self) -> (usize, usize) {
        let mut sorted = self.cards.clone();
        sorted.sort_unstable();
        let groups = sorted.windows(2).filter(|x| x[0] != x[1]).count() + 1;
        let triplets = sorted
            .windows(3)
            .filter(|x| x[0] == x[1] && x[1] == x[2])
            .count();
        (groups, triplets)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 32T3K 765
        let (cards_str, bid_str) = s.split_once(' ').ok_or(ParseHandError)?;
        let cards: [char; 5] = cards_str
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ParseHandError)?;
        let bid: u64 = bid_str.parse().map_err(|_| ParseHandError)?;
        Ok(Self { cards, bid })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_groups_triplets(groups: usize, triplets: usize) -> Self {
        match groups {
            0 | 1 => HandType::FiveOfAKind,
            2 => {
                if triplets > 1 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if triplets > 0 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn from_hand(hand: &Hand) -> HandType {
        let (groups, triplets) = hand.groups_triplets();
        HandType::from_groups_triplets(groups, triplets)
    }

    fn from_hand_jokers(hand: &Hand) -> HandType {
        let (groups, triplets) = hand.groups_triplets();
        let jokers = hand.cards.iter().filter(|&&x| x == 'J').count();
        HandType::from_groups_triplets(groups - if jokers > 0 { 1 } else { 0 }, triplets + jokers)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RankedHand {
    htype: HandType,
    cards: [usize; 5],
    bid: u64,
}

impl RankedHand {
    const STRENGTH: &[u8] = b"AKQJT98765432";

    fn from_hand(hand: &Hand) -> Self {
        let cards = hand
            .cards
            .map(|x| Self::STRENGTH.iter().position(|&c| c as char == x).unwrap());
        let htype = HandType::from_hand(hand);
        Self {
            htype,
            cards,
            bid: hand.bid,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct JokerHand {
    htype: HandType,
    cards: [usize; 5],
    bid: u64,
}

impl JokerHand {
    const STRENGTH: &[u8] = b"AKQT98765432J";

    fn from_hand(hand: &Hand) -> Self {
        let cards = hand
            .cards
            .map(|x| Self::STRENGTH.iter().position(|&c| c as char == x).unwrap());
        let htype = HandType::from_hand_jokers(hand);
        Self {
            htype,
            cards,
            bid: hand.bid,
        }
    }
}

fn main() -> io::Result<()> {
    let hands: Vec<Hand> = io::stdin()
        .lines()
        .map(|s| s.as_ref().unwrap().parse().unwrap())
        .collect();

    let mut sorted: Vec<RankedHand> = hands.iter().map(|h| RankedHand::from_hand(h)).collect();
    sorted.sort_unstable();
    let winnings: u64 = sorted
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i as u64 + 1) * x.bid)
        .sum();
    println!("{}", winnings);

    let mut sorted: Vec<JokerHand> = hands.iter().map(|h| JokerHand::from_hand(h)).collect();
    sorted.sort_unstable();
    let winnings: u64 = sorted
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i as u64 + 1) * x.bid)
        .sum();
    println!("{}", winnings);

    Ok(())
}
