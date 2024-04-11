use std::f64::consts::PI;

fn main() {}

pub enum Shape {
    Circle(i64, i64, i64),
    Rectangle(i64, i64, i64, i64),
}

impl Shape {
    fn rep_string(&self) -> String {
        match self {
            Shape::Circle(x, y, r) => format!("<Circle: {}, {}, {}>", x, y, r),
            Shape::Rectangle(x, y, w, h) => format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h),
        }
    }

    fn area(&self) -> f64 {
        match self {
            Shape::Circle(_, _, r) => PI * (r * r) as f64,
            Shape::Rectangle(_, _, w, h) => (w * h) as f64,
        }
    }
}

// fn input_shape_list() -> Vec<Box<dyn Shape>> {
//     vec![
//         Circle::new(0, 0, 1),
//         Circle::new(50, 50, 15),
//         Rectangle::new(40, 40, 20, 20),
//         Rectangle::new(10, 40, 15, 10),
//     ]
// }

const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle(0, 0, 1),
    Shape::Circle(50, 50, 15),
    Shape::Rectangle(40, 40, 20, 20),
    Shape::Rectangle(10, 40, 15, 10),
];
const EXPECTED: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
];

#[test]
fn test_shapes() {
    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list
        .iter()
        .map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}