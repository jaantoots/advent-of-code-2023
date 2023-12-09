use std::collections::HashMap;
use std::io;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if b > a {
        (a, b) = (b, a);
    }
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();

    let instructions = lines.next().unwrap()?;
    lines.next();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    while let Some(line) = lines.next() {
        let (node, edges) = line.as_ref().unwrap().split_once('=').unwrap();
        let (left, right) = edges
            .trim()
            .trim_matches('(')
            .trim_matches(')')
            .split_once(',')
            .unwrap();
        network.insert(
            node.trim().to_string(),
            (left.trim().to_string(), right.trim().to_string()),
        );
    }

    let mut steps = 0u64;
    let mut current = "AAA";
    for instruction in instructions.chars().cycle() {
        if current == "ZZZ" {
            break;
        }
        let (left, right) = network.get(current).unwrap();
        current = if instruction == 'L' { left } else { right };
        steps += 1;
    }
    println!("{}", steps);

    let mut periods: Vec<u64> = Vec::new();
    // Assume start is included in cycles, otherwise more difficult (!)
    for start in network.keys().filter(|k| k.ends_with('A')) {
        let mut steps = 0u64;
        let mut current = start;
        let mut first_end: HashMap<(&str, usize), u64> = HashMap::new();
        for (i, instruction) in instructions.chars().enumerate().cycle() {
            if current.ends_with('Z') {
                let first = first_end.entry((current, i)).or_insert(steps);
                if first != &steps {
                    let period = steps - *first;
                    periods.push(period);
                    // Check the assumption
                    println!(
                        "{} {} {} {} {} : {}",
                        start, current, i, first, steps, period
                    );
                    break;
                }
            }
            let (left, right) = network.get(current).unwrap();
            current = if instruction == 'L' { left } else { right };
            steps += 1;
        }
    }
    println!(
        "{}",
        periods.iter().fold(1, |acc, &e| acc * (e / gcd(acc, e)))
    );

    Ok(())
}
