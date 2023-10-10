use std::default;

#[derive(Default,Debug, Clone, Copy)]
struct Point{
    x:i32,
    y:i32
}

#[derive(Debug, Clone)]
struct Polyline{
    line: Vec<Point>
}

fn main() {
    let p1: Point = Default::default();
    let p2: Point = Point { x: 1, y: 2 };
    let pl = Polyline{line:vec![
        p1,
        p1.clone(),
        p2.clone()
        ]
    };
    println!("{:?}", pl.clone());
}
