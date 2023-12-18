#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    tag: String,
}

fn tag_points1(pt_list: Vec<Point>) -> Vec<Point> {
    let mut tag_list = Vec::new();

    for point in pt_list {
        if ((point.x).powf(2.) + (point.y).powf(2.)).powf(0.5) <= 1.0 {
            tag_list.push(Point {
                x: point.x,
                y: point.y,
                tag: "#80FF8080".to_string(),
            });
        } else {
            tag_list.push(Point {
                x: point.x,
                y: point.y,
                tag: "#FF808080".to_string(),
            });
        }
    }
    tag_list
}

fn main() {
    let pt_list = vec![
        Point {
            x: 1.0,
            y: 2.0,
            tag: String::new(),
        },
        Point {
            x: 1.0,
            y: -1.0,
            tag: String::new(),
        },
        Point {
            x: 0.5,
            y: -0.5,
            tag: String::new(),
        },
        Point {
            x: 1.0,
            y: 0.0,
            tag: String::new(),
        },
    ];

    let x = tag_points1(pt_list);
    println!("{:?}", x);
}

#[test]
fn test_tag_point() {
    let pt_list = vec![
        Point {
            x: 1.0,
            y: 2.0,
            tag: String::new(),
        },
        Point {
            x: 1.0,
            y: -1.0,
            tag: String::new(),
        },
        Point {
            x: 0.5,
            y: -0.5,
            tag: String::new(),
        },
        Point {
            x: 1.0,
            y: 0.0,
            tag: String::new(),
        },
    ];
    let points = tag_points1(pt_list);
    assert_eq!(points[0].tag, "#FF808080");
    assert_eq!(points[1].tag, "#FF808080");
    assert_eq!(points[2].tag, "#80FF8080");
    assert_eq!(points[3].tag, "#80FF8080");
}
