use std::ops::{Add, AddAssign, Sub, SubAssign};

use tracing::info;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn most<'a>(left: &'a Point, right: &'a Point) -> &'a Point {
    if left.x < right.x { left } else { right }
}
pub fn lifetime_run() {
    let left = Point { x: 10, y: 10 };

    {
        let most_point: &Point;
        let right = Point { x: 20, y: 20 };
        most_point = most(&left, &right);
        info!("most {:?}", most_point);
    }

    let add_point = left + Point { x: 110, y: 10 };
    info!("add_point {:?}", add_point);
    let mut add_point = add_point + (51, 90);
    info!("add_point tuple {:?}", add_point);
    add_point += Point { x: 10, y: 10 };
    info!("add_assign {:?}", add_point);
    add_point += (50, 90);
    info!("add_assign tuple {:?}", add_point);
    let minus_point = add_point - (50, 90);
    info!("minus_point tuple {:?}", minus_point);
    let minus_point = minus_point - Point { x: 10, y: 10 };
    info!("minus_point2 {:?}", minus_point);
    let doc = "Hello world!".to_owned();
    let highlight = Highlight(&doc[0..5]);
    let highlight2 = Highlight(&doc[6..11]);
    info!("highlight {:?}", highlight);
    // erase(doc); // error: cannot move out of borrowed content
    info!("highlight2 {:?}", highlight2);
    erase(doc);

    let mut a = vec![1, 2, 3, 4, 5];
    a.push(10);
    let b = &mut a[0..2];
    b[0] = 100;
    info!("b {:?}", b);
    info!("a {:?}", a);
}

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);
fn erase(text: String) {
    info!("erase {:?}", text);
}

impl Drop for Point {
    fn drop(&mut self) {
        info!("dropping Point at x={}, y={}", self.x, self.y);
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Point;
    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Point {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<(i32, i32)> for Point {
    fn sub_assign(&mut self, rhs: (i32, i32)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

#[cfg(test)]
mod lifetime_tests {
    use super::*;

    #[test]
    fn test_add() {
        let p = Point { x: 10, y: 10 };
        let q = Point { x: 20, y: 20 };
        let p = p + q;
        assert_eq!(p.x, 30);
        assert_eq!(p.y, 30);
        let p = p + (20, 20);
        assert_eq!(p.x, 50);
        assert_eq!(p.y, 50);
    }

    #[test]
    fn test_add_assign() {
        let mut p = Point { x: 10, y: 10 };
        p += Point { x: 20, y: 20 };
        assert_eq!(p.x, 30);
        assert_eq!(p.y, 30);
        p += (20, 20);
        assert_eq!(p.x, 50);
        assert_eq!(p.y, 50);
    }

    #[test]
    fn test_sub() {
        let p = Point { x: 10, y: 10 };
        let q = Point { x: 20, y: 20 };
        let p = p - q;
        assert_eq!(p.x, -10);
        assert_eq!(p.y, -10);
        let p = p - (20, 20);
        assert_eq!(p.x, -30);
        assert_eq!(p.y, -30);
    }

    #[test]
    fn test_sub_assign() {
        let mut p = Point { x: 10, y: 10 };
        p -= Point { x: 20, y: 20 };
        assert_eq!(p.x, -10);
        assert_eq!(p.y, -10);
        p -= (20, 20);
        assert_eq!(p.x, -30);
        assert_eq!(p.y, -30);
    }
}
