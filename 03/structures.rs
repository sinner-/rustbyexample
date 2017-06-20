use std::fmt::{Formatter, Display, Result};
struct Nil;

struct Pair(i32, f32);

#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "p1: {}, p2: {}", self.p1, self.p2)
    }
}

fn rect_area(r: Rectangle) -> f32 {
    let Point { x: x_p1, y: y_p1 } = r.p1;
    let Point { x: x_p2, y: y_p2 } = r.p2;
    return (x_p2 - x_p1) * (y_p2 - y_p1);
}

fn square(p: Point, wl: f32) -> Rectangle {
    return Rectangle { p2: Point { x: p.x+wl, y: p.y+wl}, p1: p };
}

fn main() {
    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point, 
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rectangle: Rectangle = Rectangle{ p1: Point { x: 1.0, y: 2.0 }, p2: Point { x: 2.3, y: 4.2} };

    println!("{}", rectangle);

    println!("{}", rect_area(rectangle));

    println!("{}", square(point, 1.1))
}
