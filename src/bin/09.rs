use std::io;

fn predict(history: &[i64]) -> i64 {
    if history.iter().all(|&x| x == 0) {
        return 0;
    }
    let diffs = history.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
    history.last().unwrap_or(&0) + predict(&diffs)
}

fn predict_back(history: &[i64]) -> i64 {
    if history.iter().all(|&x| x == 0) {
        return 0;
    }
    let diffs = history.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
    history.first().unwrap_or(&0) - predict_back(&diffs)
}

fn main() -> io::Result<()> {
    let histories: Vec<Vec<i64>> = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let sum_extrapolated: i64 = histories.iter().map(|x| predict(x)).sum();
    println!("{}", sum_extrapolated);

    let sum_extrapolated: i64 = histories.iter().map(|x| predict_back(x)).sum();
    println!("{}", sum_extrapolated);

    Ok(())
}
