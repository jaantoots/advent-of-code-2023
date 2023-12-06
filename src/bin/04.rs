use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn parse_line(s: &str) -> (u32, Vec<u32>, Vec<u32>) {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let (prefix, suffix) = s.split_once(':').unwrap();
    // Card 1
    let id: u32 = prefix
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    // 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let (win, have) = suffix.split_once('|').unwrap();
    (
        id,
        win.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        have.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
    )
}

fn card_matches(win: &[u32], have: &[u32]) -> usize {
    let wins = win.iter().collect::<HashSet<_>>();
    have.iter().filter(|x| wins.contains(x)).count()
}

fn main() -> io::Result<()> {
    let lines = io::stdin().lines();

    let mut points = 0u32;
    let mut total_cards = 0u32;
    let mut extras: VecDeque<u32> = VecDeque::new();
    for (_, win, have) in lines.map(|s| parse_line(s.as_ref().unwrap())) {
        let count = card_matches(&win, &have);
        points += count.checked_sub(1).and_then(|x| Some(1 << x)).unwrap_or(0);
        let instances = 1 + extras.pop_front().unwrap_or(0);
        total_cards += instances;
        for i in 0..count {
            if let Some(x) = extras.get_mut(i) {
                *x += instances;
            } else {
                extras.push_back(instances);
            }
        }
    }
    println!("{}", points);
    println!("{}", total_cards);

    Ok(())
}
