use std::io;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCubesError;

impl FromStr for Cubes {
    type Err = ParseCubesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 3 blue, 4 red
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for cube_str in s.split(',') {
            let (num, colour) = cube_str.trim().split_once(' ').ok_or(ParseCubesError)?;
            let i = num.parse().map_err(|_| ParseCubesError)?;
            match colour {
                "red" => red = i,
                "green" => green = i,
                "blue" => blue = i,
                _ => return Err(ParseCubesError),
            }
        }
        Ok(Cubes { red, green, blue })
    }
}

const BAG: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn parse_line(s: &str) -> (u32, Vec<Cubes>) {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (game, sets) = s.split_once(':').unwrap();
    // Game 1
    let id: u32 = game.split(' ').last().unwrap().parse().unwrap();
    // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let sets = sets
        .split(';')
        .map(|x| x.parse::<Cubes>().unwrap())
        .collect::<Vec<_>>();
    (id, sets)
}

fn main() -> io::Result<()> {
    let lines = io::stdin().lines();

    let games = lines
        .map(|l| parse_line(l.as_ref().unwrap()))
        .collect::<Vec<_>>();

    let sum: u32 = games
        .iter()
        .filter_map(|(id, sets)| {
            sets.iter()
                .all(|x| x.red <= BAG.red && x.green <= BAG.green && x.blue <= BAG.blue)
                .then_some(id)
        })
        .sum();
    println!("{}", sum);

    let power_sum: u32 = games
        .iter()
        .map(|(_, sets)| {
            sets.iter().fold(Cubes::default(), |x, y| Cubes {
                red: x.red.max(y.red),
                green: x.green.max(y.green),
                blue: x.blue.max(y.blue),
            })
        })
        .map(|x| x.red * x.green * x.blue)
        .sum();
    println!("{}", power_sum);

    Ok(())
}
