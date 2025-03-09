use std::vec;

use tracing::info;

pub fn iterator_run() {
    info!("iterator_run");

    let fib = Fibonacci { curr: 0, next: 5 };

    for (i, n) in fib.enumerate().take(5) {
        info!("fib({}) = {}", i, n);
    }

    let grid = Grid {
        x_coords: vec![3, 5, 7, 9],
        y_coords: vec![10, 20, 30, 40],
    };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }

    let primes = vec![2, 3, 5, 7];
    let primes_squares = primes.iter().map(|&x| x * x).collect::<Vec<_>>();
    info!("primes_squares {:?}", primes_squares);
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = dbg!(self.next);
        self.next = dbg!(new_next);
        Some(self.curr)
    }
}

struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

struct GridIterator {
    x: u32,
    y: u32,
    grid: Grid,
}

impl Iterator for GridIterator {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.x_coords.is_empty() || self.grid.y_coords.is_empty() {
            return None;
        }
        if self.y < self.grid.y_coords.len() as u32 {
            let y = self.grid.y_coords[self.y as usize];
            self.y += 1;
            return Some((self.grid.x_coords[self.x as usize], y));
        }

        if (self.x + 1) < self.grid.x_coords.len() as u32 {
            self.y = 0;
            self.x += 1;
            return Some((
                self.grid.x_coords[self.x as usize],
                self.grid.y_coords[self.y as usize],
            ));
        }

        None
    }
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIterator;
    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            x: 0,
            y: 0,
            grid: self,
        }
    }
}
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences1<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: std::ops::Sub<Output = N> + Copy,
{
    values
        .iter()
        .enumerate()
        .map(|(i, v)| values[(i + offset) % values.len()] - *v)
        .collect::<Vec<N>>()
}

fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: std::ops::Sub<Output = N> + Copy,
{
    values
        .iter()
        .zip(values.iter().cycle().skip(offset))
        .map(|(a, b)| *b - *a)
        .collect::<Vec<N>>()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    )
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<u32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}

#[test]
fn test_cycle_skip() {
    let v = vec![1, 2, 3, 4];
    let mut iter = v.iter().cycle().skip(2);
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(v.iter().skip(2).cycle().next(), Some(&3));
}
