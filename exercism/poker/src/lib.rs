struct Ranks {
    ranks: Vec<u8>,
}
impl Ranks {
    fn from_cards(cards: &Vec<(u8, u8)>, picks: &[usize]) -> Self {
        Self {
            ranks: picks.iter().map(|idx| cards[*idx].0).collect(),
        }
    }
}
impl PartialEq for Ranks {
    fn eq(&self, other: &Self) -> bool {
        self.ranks.eq(&other.ranks)
    }
}
impl PartialOrd for Ranks {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ranks.partial_cmp(&other.ranks)
    }
}

enum HandRankingCategory {
    // StraightFlush(5) = A, 2, 3, 4, 5
    // StraightFlush(14) = 10, J, Q, K, A
    StraightFlush(Ranks),
    // FourOfAKind(14, 13) = A, A, A, A, K
    FourOfAKind(Ranks),
    // FullHouse(3, 2) = 3, 3, 3, 2, 2
    FullHouse(Ranks),
    // Flush(14, 10, 8, 5, 2) = AH, 10H, 8H, 5H, 2H
    Flush(Ranks),
    // StraightFlush(5) = A, 2, 3, 4, 5
    // StraightFlush(14) = 10, J, Q, K, A
    Straight(Ranks),
    // ThreeOfAKind(3, 7, 2) = 3, 3, 3, 7, 2
    ThreeOfAKind(Ranks),
    TwoPair(Ranks),
    OnePair(Ranks),
    HighCard(Ranks),
}
impl HandRankingCategory {
    fn new(hand: &str) -> Self {
        let mut cards: Vec<(u8, u8)> = hand
            .split(" ")
            .map(|rank_and_suit| {
                let (rank, suit) = rank_and_suit.split_at(rank_and_suit.len() - 1);
                let suit: u8 = suit.as_bytes()[0];
                match rank {
                    "10" => (10, suit),
                    "J" => (11, suit),
                    "Q" => (12, suit),
                    "K" => (13, suit),
                    "A" => (14, suit),
                    _ => (rank.parse().unwrap(), suit),
                }
            })
            .collect();
        cards.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let is_straight = cards[1].0 == cards[0].0 + 1
            && cards[2].0 == cards[0].0 + 2
            && cards[3].0 == cards[0].0 + 3
            && cards[4].0 == cards[0].0 + 4;
        let is_straight_start_from_ace = cards[0].0 == 2
            && cards[1].0 == 3
            && cards[2].0 == 4
            && cards[3].0 == 5
            && cards[4].0 == 14;
        let is_flush = cards[1].1 == cards[0].1
            && cards[2].1 == cards[0].1
            && cards[3].1 == cards[0].1
            && cards[4].1 == cards[0].1;

        if is_straight_start_from_ace {
            // [2, 3, 4, 5, A]
            if is_flush {
                return Self::StraightFlush(Ranks::from_cards(&cards, &[3]));
            }
            return Self::Straight(Ranks::from_cards(&cards, &[3]));
        }
        if is_straight {
            // [2, 3, 4, 5, 6], [10, J, Q, K, A]
            if is_flush {
                return Self::StraightFlush(Ranks::from_cards(&cards, &[4]));
            }
            return Self::Straight(Ranks::from_cards(&cards, &[4]));
        }

        if cards[0].0 == cards[1].0 && cards[0].0 == cards[2].0 && cards[0].0 == cards[3].0 {
            // [2, 2, 2, 2, A]
            return Self::FourOfAKind(Ranks::from_cards(&cards, &[0, 4]));
        } else if cards[4].0 == cards[1].0 && cards[4].0 == cards[2].0 && cards[4].0 == cards[3].0 {
            // [2, A, A, A, A]
            return Self::FourOfAKind(Ranks::from_cards(&cards, &[1, 0]));
        }

        if cards[0].0 == cards[1].0 && cards[0].0 == cards[2].0 {
            if cards[3].0 == cards[4].0 {
                // [2, 2, 2, 3, 3]
                return Self::FullHouse(Ranks::from_cards(&cards, &[0, 3]));
            }
            // [2, 2, 2, 3, 4]
            return Self::ThreeOfAKind(Ranks::from_cards(&cards, &[0, 4, 3]));
        } else if cards[1].0 == cards[2].0 && cards[1].0 == cards[3].0 {
            // [2, 3, 3, 3, 4]
            return Self::ThreeOfAKind(Ranks::from_cards(&cards, &[1, 4, 0]));
        } else if cards[2].0 == cards[3].0 && cards[2].0 == cards[4].0 {
            if cards[0].0 == cards[1].0 {
                // [2, 2, 3, 3, 3]
                return Self::FullHouse(Ranks::from_cards(&cards, &[2, 0]));
            }
            // [2, 3, 4, 4, 4]
            return Self::ThreeOfAKind(Ranks::from_cards(&cards, &[2, 1, 0]));
        }

        if is_flush {
            return Self::Flush(Ranks::from_cards(&cards, &[4, 3, 2, 1, 0]));
        }

        if cards[0].0 == cards[1].0 {
            if cards[2].0 == cards[3].0 {
                // [2, 2, 3, 3, 4]
                return Self::TwoPair(Ranks::from_cards(&cards, &[2, 0, 4]));
            } else if cards[3].0 == cards[4].0 {
                // [2, 2, 3, 4, 4]
                return Self::TwoPair(Ranks::from_cards(&cards, &[3, 0, 2]));
            } else {
                // [2, 2, 3, 4, 5]
                return Self::OnePair(Ranks::from_cards(&cards, &[0, 4, 3, 2]));
            }
        } else if cards[1].0 == cards[2].0 {
            if cards[3].0 == cards[4].0 {
                // [2, 3, 3, 4, 4]
                return Self::TwoPair(Ranks::from_cards(&cards, &[4, 2, 0]));
            } else {
                // [2, 3, 3, 4, 5]
                return Self::OnePair(Ranks::from_cards(&cards, &[1, 4, 3, 0]));
            }
        } else if cards[2].0 == cards[3].0 {
            // [2, 3, 4, 4, 5]
            return Self::OnePair(Ranks::from_cards(&cards, &[2, 4, 1, 0]));
        } else if cards[3].0 == cards[4].0 {
            // [2, 3, 4, 5, 5]
            return Self::OnePair(Ranks::from_cards(&cards, &[3, 2, 1, 0]));
        }

        Self::HighCard(Ranks::from_cards(&cards, &[4, 3, 2, 1, 0]))
    }
    /// hands with smaller rank win
    fn get_rank(&self) -> u8 {
        match self {
            Self::StraightFlush(_) => 0,
            Self::FourOfAKind(_) => 1,
            Self::FullHouse(_) => 2,
            Self::Flush(_) => 3,
            Self::Straight(_) => 4,
            Self::ThreeOfAKind(_) => 5,
            Self::TwoPair(_) => 6,
            Self::OnePair(_) => 7,
            Self::HighCard(_) => 8,
        }
    }
    fn get_ranks(&self) -> &Ranks {
        match self {
            HandRankingCategory::StraightFlush(ranks) => &ranks,
            HandRankingCategory::FourOfAKind(ranks) => &ranks,
            HandRankingCategory::FullHouse(ranks) => &ranks,
            HandRankingCategory::Flush(ranks) => &ranks,
            HandRankingCategory::Straight(ranks) => &ranks,
            HandRankingCategory::ThreeOfAKind(ranks) => &ranks,
            HandRankingCategory::TwoPair(ranks) => &ranks,
            HandRankingCategory::OnePair(ranks) => &ranks,
            HandRankingCategory::HighCard(ranks) => &ranks,
        }
    }
}
impl PartialEq for HandRankingCategory {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StraightFlush(l0), Self::StraightFlush(r0)) => l0 == r0,
            (Self::FourOfAKind(l0), Self::FourOfAKind(r0)) => l0 == r0,
            (Self::FullHouse(l0), Self::FullHouse(r0)) => l0 == r0,
            (Self::Flush(l0), Self::Flush(r0)) => l0 == r0,
            (Self::Straight(l0), Self::Straight(r0)) => l0 == r0,
            (Self::ThreeOfAKind(l0), Self::ThreeOfAKind(r0)) => l0 == r0,
            (Self::TwoPair(l0), Self::TwoPair(r0)) => l0 == r0,
            (Self::OnePair(l0), Self::OnePair(r0)) => l0 == r0,
            (Self::HighCard(l0), Self::HighCard(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl PartialOrd for HandRankingCategory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.get_rank() == other.get_rank() {
            return self.get_ranks().partial_cmp(&other.get_ranks());
        }
        match self.get_rank() < other.get_rank() {
            true => Some(std::cmp::Ordering::Greater),
            false => Some(std::cmp::Ordering::Less),
        }
    }
}

struct Hand<'a> {
    ranking_category: HandRankingCategory,
    original_hand: &'a str,
}
impl<'a> Hand<'a> {
    fn new(hand: &'a str) -> Self {
        Hand {
            ranking_category: HandRankingCategory::new(hand),
            original_hand: hand,
        }
    }
}
impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ranking_category.eq(&other.ranking_category)
    }
}
impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ranking_category.partial_cmp(&other.ranking_category)
    }
}
impl<'a> Eq for Hand<'a> {}
impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ranking_category
            .partial_cmp(&other.ranking_category)
            .unwrap()
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        return vec![hands[0]];
    }
    let mut winners = vec![];
    let mut hands: Vec<Hand> = hands.iter().map(|&hand| Hand::new(hand)).collect();
    hands.sort();
    hands.reverse();
    winners.push(hands[0].original_hand);
    for i in 1..hands.len() {
        if hands[i] == hands[0] {
            winners.push(hands[i].original_hand);
        } else {
            return winners;
        }
    }
    winners
}
