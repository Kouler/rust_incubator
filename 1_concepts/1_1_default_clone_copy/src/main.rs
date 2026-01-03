//use smart_default::SmartDefault;

#[derive(Default, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Polyline {
    first: Point,
    rest: Vec<Point>,
}

impl Polyline {

    fn new(first: Point, rest: Vec<Point>) -> Self {
        Self { first, rest }
    }

    fn from_point(first: Point) -> Self {
        Self {
            first,
            rest: Vec::new(),
        }
    }

    fn from_slice(points: &[Point]) -> Option<Self> {
        if points.is_empty() {
            None
        }
        else {
            let first = points[0];
            let rest = points[1..].to_vec();
            Some ( Self {
                first,
                rest
            })
        }
    }

    fn points(&self) -> impl Iterator<Item = &Point> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    fn push(&mut self, point: Point) {
        self.rest.push(point);
    }
}


fn main() {

}
