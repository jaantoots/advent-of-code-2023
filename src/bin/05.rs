use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::io;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Default)]
struct Range<T> {
    dst: T,
    src: T,
    len: T,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;

impl<T> FromStr for Range<T>
where
    T: FromStr + Copy,
{
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 50 98 2
        let it: Vec<T> = s
            .split_ascii_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        if it.len() != 3 {
            Err(ParseRangeError)
        } else {
            Ok(Range {
                dst: it[0],
                src: it[1],
                len: it[2],
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct SimpleState<T: Ord> {
    src: BTreeSet<T>,
    dst: BTreeSet<T>,
}

impl<T> SimpleState<T>
where
    T: Ord + Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn map_range(&mut self, range: &Range<T>) {
        let mapped: Vec<_> = self
            .src
            .range(range.src..range.src + range.len)
            .copied()
            .collect();
        for el in mapped {
            self.dst.insert(range.dst + (el - range.src));
            self.src.remove(&el);
        }
    }

    pub fn finish(&mut self) {
        self.src.append(&mut self.dst);
    }
}

impl<T> FromIterator<T> for SimpleState<T>
where
    T: Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            src: BTreeSet::from_iter(iter),
            dst: BTreeSet::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct RangeMap<T: Ord> {
    tree: BTreeMap<T, T>,
}

impl<T> RangeMap<T>
where
    T: Ord + Copy,
{
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, x: T, y: T) {
        self.tree.insert(x, y);
        self.tree.insert(y, x);
    }

    pub fn remove(&mut self, x: &T) {
        let y = self.tree.remove(x);
        if let Some(y) = y {
            self.tree.remove(&y);
        }
    }
}

impl<T> FromIterator<T> for RangeMap<T>
where
    T: Ord + Copy + Add<Output = T> + Sub<Output = T> + From<u64>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut it = iter.into_iter();
        let mut tree = BTreeMap::new();
        while let (Some(start), Some(length)) = (it.next(), it.next()) {
            let end = start + length - 1.into();
            tree.insert(start, end);
            tree.insert(end, start);
        }
        Self { tree }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct RangeState<T: Ord> {
    src: RangeMap<T>,
    dst: RangeMap<T>,
}

impl<T> RangeState<T>
where
    T: Ord + Copy + Add<Output = T> + Sub<Output = T> + From<u64> + Debug,
{
    pub fn map_range(&mut self, range: &Range<T>) {
        let mapped: Vec<_> = self
            .src
            .tree
            .range(range.src..range.src + range.len)
            .map(|(&x, &y)| (x, y))
            .collect();
        for (x, y) in mapped {
            if x <= y {
                if y < range.src + range.len {
                    self.dst
                        .insert(range.dst + (x - range.src), range.dst + (y - range.src));
                    self.src.remove(&x);
                } else {
                    self.dst.insert(
                        range.dst + (x - range.src),
                        range.dst + range.len - 1.into(),
                    );
                    self.src.remove(&x);
                    self.src.insert(range.src + range.len, y);
                }
            } else {
                if y < range.src {
                    self.dst.insert(range.dst, range.dst + (x - range.src));
                    self.src.remove(&x);
                    self.src.insert(y, range.src - 1.into());
                }
                // already done by other pair
            }
        }
        if let Some((x, y)) = self
            .src
            .tree
            .range(..range.src)
            .last()
            .map(|(&x, &y)| (x, y))
        {
            if y >= range.src + range.len {
                self.dst.insert(range.dst, range.dst + range.len - 1.into());
                self.src.remove(&x);
                self.src.insert(x, range.src - 1.into());
                self.src.insert(range.src + range.len, y);
            }
        }
    }

    pub fn finish(&mut self) {
        self.src.tree.append(&mut self.dst.tree);
    }
}

impl<T> FromIterator<T> for RangeState<T>
where
    T: Ord + Copy + Add<Output = T> + Sub<Output = T> + From<u64>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            src: RangeMap::from_iter(iter),
            dst: RangeMap::new(),
        }
    }
}

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines();

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()?
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut simple_state = SimpleState::from_iter(seeds.iter().copied());
    let mut range_state = RangeState::from_iter(seeds.iter().copied());
    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            simple_state.finish();
            range_state.finish();
            // next block
            let _line = lines.next();
            continue;
        }
        let range = Range::from_str(&line).unwrap();
        simple_state.map_range(&range);
        range_state.map_range(&range);
    }
    simple_state.finish();
    range_state.finish();
    println!("{}", simple_state.src.first().unwrap());
    println!("{}", range_state.src.tree.first_key_value().unwrap().0);

    Ok(())
}
