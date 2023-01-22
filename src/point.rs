use std::convert::From;
use std::ops::{Add, Mul, Sub};

use graphics::math::{Scalar, Vec2d};

use crate::Size;

/// A point in the Cartesian plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// The x coordinate.
    pub x: Scalar,
    /// The y coordinate.
    pub y: Scalar,
}

impl Add<Scalar> for Point {
    type Output = Point;

    fn add(self, s: Scalar) -> Point {
        Point {
            x: self.x + s,
            y: self.y + s,
        }
    }
}

impl<T: Into<Point>> Add<T> for Point {
    type Output = Point;

    fn add(self, v: T) -> Point {
        let v: Point = v.into();
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl From<Point> for Vec2d {
    fn from(point: Point) -> Vec2d {
        [point.x, point.y]
    }
}

impl From<Vec2d> for Point {
    fn from(v: Vec2d) -> Point {
        Point { x: v[0], y: v[1] }
    }
}

impl From<(Scalar, Scalar)> for Point {
    fn from((x, y): (Scalar, Scalar)) -> Point {
        Point { x: x, y: y }
    }
}

impl From<Size> for Point {
    fn from(size: Size) -> Point {
        Point {
            x: size.w,
            y: size.h,
        }
    }
}

impl Sub<Scalar> for Point {
    type Output = Point;

    fn sub(self, s: Scalar) -> Point {
        Point {
            x: self.x - s,
            y: self.y - s,
        }
    }
}

impl<T: Into<Point>> Sub<T> for Point {
    type Output = Point;

    fn sub(self, v: T) -> Point {
        let v: Point = v.into();
        Point {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, s: Scalar) -> Self::Output {
        Point {
            x: self.x * s,
            y: self.y * s,
        }
    }
}
