use std::io;

fn get_value(s: &str) -> Option<u32> {
    let mut digits = s.chars().filter(|c| c.is_ascii_digit()).peekable();
    Some(digits.peek().as_deref()?.to_digit(10)? * 10 + digits.last()?.to_digit(10)?)
}

const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const REPL: &[&str] = &[
    "one1one",
    "two2two",
    "three3three",
    "four4four",
    "five5five",
    "six6six",
    "seven7seven",
    "eight8eight",
    "nine9nine",
];

/*
use std::iter::successors;
fn parse_digit(s: &str) -> Option<String> {
    let x = DIGITS
        .iter()
        .map(|&x| s.find(x))
        .enumerate()
        .filter_map(|(i, x)| x.and_then(|y| Some((i, y))))
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .map(|(i, _)| i);
    x.and_then(|idx| Some(s.replacen(DIGITS[idx], &(idx + 1).to_string(), 1)))
}
let fixed = successors(Some(s.to_string()), |x| parse_digit(x)).last().unwrap();
*/

fn get_real_value(s: &str) -> Option<u32> {
    let fixed = DIGITS
        .iter()
        .zip(REPL.iter())
        .fold(s.to_string(), |t, (&from, &to)| t.replace(from, to));
    get_value(&fixed)
}

fn main() -> io::Result<()> {
    let lines = io::stdin().lines().collect::<Vec<_>>();

    let sum = lines.iter().fold(0, |acc, line| {
        acc + get_value(line.as_deref().unwrap_or("")).unwrap_or(0)
    });
    println!("{}", sum);

    let real_sum = lines.iter().fold(0, |acc, line| {
        acc + get_real_value(line.as_deref().unwrap_or("")).unwrap_or(0)
    });
    println!("{}", real_sum);

    Ok(())
}
