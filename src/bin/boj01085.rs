#[derive(Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct LineX {
    x: usize,
}

impl LineX {
    pub fn new(x: usize) -> Self {
        Self { x }
    }

    pub fn distance(&self, p: &Point) -> usize {
        if p.x == self.x {
            0
        } else if p.x < self.x {
            self.x - p.x
        } else {
            p.x - self.x
        }
    }
}

pub struct LineY {
    y: usize,
}

impl LineY {
    pub fn new(y: usize) -> Self {
        Self { y }
    }

    pub fn distance(&self, p: &Point) -> usize {
        if (p.y == self.y) {
            0
        } else if (p.y < self.y) {
            self.y - p.y
        } else {
            p.y - self.y
        }
    }
}

pub struct RectangleWithOrigin {
    diag_point: Point,
    x_line: [LineX; 2],
    y_line: [LineY; 2],
}

impl RectangleWithOrigin {
    pub fn new(diag_point: Point) -> Self {
        Self {
            diag_point,
            x_line: [LineX::new(0), LineX::new(diag_point.x)],
            y_line: [LineY::new(0), LineY::new(diag_point.y)],
        }
    }

    pub fn distance(&self, p: &Point) -> usize {
        let x_dists = [self.x_line[0].distance(p), self.x_line[1].distance(p)];
        let y_dists = [self.y_line[0].distance(p), self.y_line[1].distance(p)];
        x_dists[0].min(x_dists[1]).min(y_dists[0].min(y_dists[1]))
    }
}

use algorithm::io::{Reader, Writer};
fn main() {
    let (mut reader, mut writer) = (Reader::new(), Writer::new());
    let (x, y, w, h) = (
        reader.next::<usize>(),
        reader.next::<usize>(),
        reader.next::<usize>(),
        reader.next::<usize>(),
    );

    let pt = Point::new(x, y);
    let diag_pt = Point::new(w, h);
    let rect = RectangleWithOrigin::new(diag_pt);

    writer.write(rect.distance(&pt));
}
