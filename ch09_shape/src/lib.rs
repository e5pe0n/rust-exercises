use std::{f32::consts::PI, ops::MulAssign};

pub struct Circle<T: num::Num> {
    radius: T,
}

impl<T: num::Num + num::NumCast + Copy + MulAssign> Circle<T> {
    pub fn new(radius: T) -> Self {
        Circle { radius }
    }

    pub fn area(&self) -> f32 {
        num::cast::<T, f32>(self.radius * self.radius).unwrap() * PI
    }

    pub fn scale(&mut self, ratio: T) {
        self.radius *= ratio;
    }

    pub fn destroy(self) -> T {
        self.radius
    }
}

pub struct Square<T: num::Num> {
    side: T,
}

impl<T: num::Num + num::NumCast + Copy + MulAssign> Square<T> {
    pub fn new(side: T) -> Self {
        Square { side }
    }

    pub fn area(&self) -> f32 {
        num::cast(self.side * self.side).unwrap()
    }

    pub fn scale(&mut self, ratio: T) {
        self.side *= ratio;
    }

    pub fn destroy(self) -> T {
        self.side
    }
}

trait HasArea {
    fn area(&self) -> f32;
}

impl<T: num::Num + num::NumCast + Copy + MulAssign> HasArea for Circle<T> {
    fn area(&self) -> f32 {
        Circle::area(self)
    }
}

impl<T: num::Num + num::NumCast + Copy + MulAssign> HasArea for Square<T> {
    fn area(&self) -> f32 {
        Square::area(self)
    }
}

enum Shape<T: num::Num + num::NumCast + Copy + MulAssign> {
    Square(Square<T>),
    Circle(Circle<T>),
}

impl<T: num::Num + num::NumCast + Copy + MulAssign> HasArea for Shape<T> {
    fn area(&self) -> f32 {
        match self {
            Self::Circle(circle) => HasArea::area(circle),
            Self::Square(square) => HasArea::area(square),
        }
    }
}
