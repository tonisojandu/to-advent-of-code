use std::collections::HashMap;
use num_bigint::BigUint;

struct Hand {
    hand_type: u64,
    second_order: u64,
    bid: u64,
}

impl Hand {
    fn new(cards: Vec<u64>, bid: u64, part: i32) -> Hand {
        let mut second_order: u64 = 0;
        let hand_type = determine_hand_type(&cards, part);
        for card in &cards {
            second_order = second_order * 100 + card;
        }
        Hand {
            hand_type,
            second_order,
            bid,
        }
    }
}

fn get_card_map(part: i32) -> HashMap<char, u64> {
    let mut card_map = HashMap::new();
    card_map.insert('2', 2);
    card_map.insert('3', 3);
    card_map.insert('4', 4);
    card_map.insert('5', 5);
    card_map.insert('6', 6);
    card_map.insert('7', 7);
    card_map.insert('8', 8);
    card_map.insert('9', 9);
    card_map.insert('T', 10);
    card_map.insert('J', if part == 2 { 1 } else { 11 });
    card_map.insert('Q', 12);
    card_map.insert('K', 13);
    card_map.insert('A', 14);
    return card_map;
}

pub fn solve(lines: Vec<String>, part: i32) {
    let card_map = get_card_map(part);

    let mut hands: Vec<Hand> = lines.iter().map(|line| {
        let mut split = line.split(" ").into_iter();
        let card_string = split.next().unwrap();
        let bid_string = split.next().unwrap();
        Hand::new(
            card_string.chars().map(|c| card_map.get(&c).unwrap().clone()).collect::<Vec<u64>>(),
            bid_string.parse::<u64>().unwrap(),
            part,
        )
    }).collect();

    hands.sort_by(|a, b| {
        let result = a.hand_type.cmp(&b.hand_type);
        if result == std::cmp::Ordering::Equal {
            return a.second_order.cmp(&b.second_order);
        }
        return result;
    });

    let total: BigUint = hands.iter().enumerate().map(|(i, hand)| {
        BigUint::from((i as u64 + 1) * hand.bid)
    }).sum();

    println!("Day 07: {} = {:?}", part, total);
}

fn determine_hand_type(cards: &Vec<u64>, part: i32) -> u64 {
    let mut similar_cards = HashMap::new();
    for card in cards {
        if part == 1 || 1 != *card {
            let count = match similar_cards.get(card) {
                Some(count) => count + 1,
                None => 1,
            };
            similar_cards.insert(card, count);
        }
    }

    let mut counts: Vec<i32> = similar_cards.values().into_iter().map(|a| a.clone()).collect();
    counts.sort();
    counts.reverse();

    // five of a kind
    if counts == vec![5] {
        return 107; // five of a kind
    }
    if counts == vec![4] {
        return 107; // five of a kind with a joker
    }
    if counts == vec![3] {
        return 107; // five of a kind with a two jokers
    }
    if counts == vec![2] {
        return 107; // five of a kind with a three jokers
    }
    if counts == vec![1] {
        return 107; // five of a kind with a four jokers
    }
    if counts == vec![] {
        return 107; // five of a kind with a five jokers
    }

    // four of a kind
    if counts == vec![4, 1] {
        return 106; // four of a kind
    }
    if counts == vec![3, 1] {
        return 106; // four of a kind with a joker
    }
    if counts == vec![2, 1] {
        return 106; // four of a kind with two jokers
    }
    if counts == vec![1, 1] {
        return 106; // four of a kind with three jokers
    }

    // full house
    if counts == vec![3, 2] {
        return 105; // full house
    }
    if counts == vec![2, 2] {
        return 105; // full house with a joker
    }

    // three of a kind
    if counts == vec![3, 1, 1] {
        return 104; // three of a kind
    }
    if counts == vec![2, 1, 1] {
        return 104; // three of a kind with a joker
    }
    if counts == vec![1, 1, 1] {
        return 104; // three of a kind with two jokers
    }

    // two pair
    if counts == vec![2, 2, 1] {
        return 103; // two pair
    }

    // one pair
    if counts == vec![2, 1, 1, 1] {
        return 102; // one pair
    }
    if counts == vec![1, 1, 1, 1] {
        return 102; // one pair with a joker
    }

    // highest card
    if counts == vec![1, 1, 1, 1, 1] {
        return 101; // highest card
    }

    panic!("Unknown hand type: {:?}", counts);
}