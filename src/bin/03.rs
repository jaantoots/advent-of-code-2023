use std::{io, iter};

fn line_symbols(s: &str) -> impl Iterator<Item = bool> + '_ {
    // 617*......
    s.chars().map(|c| !(c.is_ascii_digit() || c == '.'))
}

fn line_gear_locations(s: &str) -> impl Iterator<Item = usize> + '_ {
    // 617*......
    s.chars()
        .enumerate()
        .filter(|(_, c)| c == &'*')
        .map(|(i, _)| i)
}

fn line_numbers(s: &str) -> impl Iterator<Item = (usize, usize, u32)> + '_ {
    // 467..114..
    let mut line = s.chars().enumerate();
    iter::from_fn(move || {
        let (idxs, chars): (Vec<_>, String) = line
            .by_ref()
            .skip_while(|(_, c)| !c.is_ascii_digit())
            .take_while(|(_, c)| c.is_ascii_digit())
            .unzip();
        if idxs.is_empty() {
            None
        } else {
            Some((
                *idxs.iter().min().unwrap(),
                *idxs.iter().max().unwrap(),
                chars.parse().unwrap(),
            ))
        }
    })
}

fn main() -> io::Result<()> {
    let lines: Vec<_> = io::stdin().lines().map(|r| r.unwrap()).collect();
    let numbers: Vec<Vec<_>> = lines.iter().map(|s| line_numbers(s).collect()).collect();
    let symbols: Vec<Vec<_>> = lines.iter().map(|s| line_symbols(s).collect()).collect();

    let mut sum = 0u32;
    for (i, nums) in numbers.iter().enumerate() {
        let adj_symbols = (0..=2)
            .filter_map(|j| (i + j).checked_sub(1).and_then(|k| symbols.get(k)))
            .fold(vec![false; symbols[i].len()], |acc, e| {
                acc.iter().zip(e).map(|(x, y)| x | y).collect()
            });
        for (s, e, part) in nums {
            if adj_symbols[s.checked_sub(1).unwrap_or(0)..(e + 2).min(adj_symbols.len())]
                .iter()
                .any(|f| *f)
            {
                sum += part;
            }
        }
    }
    println!("{}", sum);

    let mut sum_ratios = 0u32;
    for (i, line) in lines.iter().enumerate() {
        for loc in line_gear_locations(line) {
            let adj: Vec<_> = (0..=2)
                .filter_map(|j| (i + j).checked_sub(1).and_then(|k| numbers.get(k)))
                .flat_map(|v| v.iter())
                .filter(|(s, e, _)| s <= &(loc + 1) && loc <= e + 1)
                .collect();
            if adj.len() == 2 {
                // ratio
                sum_ratios += adj.iter().map(|(_, _, x)| x).product::<u32>();
            }
        }
    }
    println!("{}", sum_ratios);

    Ok(())
}
