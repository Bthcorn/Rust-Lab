use rand::Rng;
use std::{error::Error, fs::File, io::Write};

#[derive(Debug)]
pub struct RandConfig {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Bound {
    pub circle1: Circle,
    pub circle2: Circle,
}

#[derive(Debug, PartialEq)]
pub enum Location {
    Both((Point, f64, f64)),
    First((Point, f64, f64)),
    Second((Point, f64, f64)),
    Outside((Point, f64, f64)),
}

// fn gen_range<T, R: Rng>(rng: &mut R, min: T, max: T) -> T
// where
//     T: PartialOrd + Copy + rand::distributions::uniform::SampleUniform,
// {
//     rng.gen_range(min..max)
// }

pub fn gen_point_list<R: Rng>(rng: &mut R, cfg: &RandConfig, n: usize) -> Vec<Point> {
    let mut points = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(cfg.x_min..cfg.x_max);
        let y = rng.gen_range(cfg.y_min..cfg.y_max);
        points.push(Point { x, y });
    }
    points
}

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn locate_n_point(b: &Bound, pt_lists: &[Point]) -> Vec<Location> {
    let mut result = Vec::new();
    let mut iter = pt_lists.iter();

    while let Some(pt) = iter.next() {
        let d1 = distance(&pt, &b.circle1.center);
        let d2 = distance(&pt, &b.circle2.center);

        if d1 <= b.circle1.radius && d2 <= b.circle2.radius {
            result.push(Location::Both((*pt, d1, d2)));
        } else if d1 <= b.circle1.radius {
            result.push(Location::First((*pt, d1, d2)));
        } else if d2 <= b.circle2.radius {
            result.push(Location::Second((*pt, d1, d2)));
        } else {
            result.push(Location::Outside((*pt, d1, d2)));
        }
    }

    result
}

pub fn map_to_svg(cfg: &RandConfig, pt: &Point) -> (i64, i64) {
    let (w, h) = (600, 600);
    let scale = (h as f64) / (cfg.y_max - cfg.y_min);
    let x = (pt.x - cfg.x_min) * scale;
    let y = (-pt.y - cfg.y_min) * scale;
    (x as i64, y as i64) // map (x, y) to SVG output

    // Implement your logic to map x to SVG coordinates here
    // For example, you can use a scale factor and offset
    // to map x to the SVG's coordinate system.
    // You might need to adjust this based on your specific SVG requirements.
    // Example: (x - x_min) * scale_factor + x_offset
}

pub fn gen_svg(cfg: &RandConfig, list: &[Location], b: &Bound) -> String {
    let mut svg_content = String::new();
    let (w, h) = (600, 600);
    let scale = (h as f64) / (cfg.y_max - cfg.y_min);
    // Create the SVG header and any other necessary SVG setup
    svg_content
        .push_str("<svg width=\"600\" height=\"600\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg_content.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"black\" stroke-width=\"2\" fill=\"none\" />\n", 
    map_to_svg(cfg, &b.circle1.center).0, map_to_svg(cfg, &b.circle1.center).1, b.circle1.radius*scale));
    svg_content.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"orange\" stroke-width=\"2\" fill=\"none\" />\n", 
    map_to_svg(cfg, &b.circle2.center).0, map_to_svg(cfg, &b.circle2.center).1, b.circle2.radius*scale));
    for loc in list {
        match loc {
            Location::Both((point, _distance, _distance1)) => {
                let pt_map = map_to_svg(&cfg, &point);
                svg_content.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"green\" />\n",
                    pt_map.0, pt_map.1
                ));
            }
            Location::First((point, _distance, _distance1)) => {
                let point2 = map_to_svg(cfg, &point);
                svg_content.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"red\" />\n",
                    point2.0, point2.1
                ));
            }
            Location::Second((point, _distance, _distance1)) => {
                let point2 = map_to_svg(cfg, &point);
                svg_content.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"blue\" />\n",
                    point2.0, point2.1
                ));
            }
            Location::Outside((point, _distance, _distance1)) => {
                let point2 = map_to_svg(cfg, &point);
                svg_content.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"purple\" />\n",
                    point2.0, point2.1
                ));
            }
        }
    }

    // Close the SVG element
    svg_content.push_str("</svg>");

    svg_content
}



#[test]
fn test_locate_point() {
    let circle1 = Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 5.0,
    };

    let circle2 = Circle {
        center: Point { x: 10.0, y: 0.0 },
        radius: 5.0,
    };

    let bound = Bound { circle1, circle2 };

    let points = vec![
        Point { x: 0.0, y: 5.0 },  // Inside both circles
        Point { x: 15.0, y: 0.0 }, // Inside first circle, outside second
        Point { x: 5.0, y: 0.0 },  // Inside second circle, outside first
        Point { x: 15.0, y: 5.0 }, // Outside both circles
    ];

    let result = locate_n_point(&bound, &points);

    assert_eq!(result.len(), 4);
    assert_eq!(
        result[0],
        Location::First((Point { x: 0.0, y: 5.0 }, 5.0, 11.180339887498949))
    );
    assert_eq!(
        result[1],
        Location::Second((Point { x: 15.0, y: 0.0 }, 15.0, 5.0))
    );
    assert_eq!(
        result[2],
        Location::Both((Point { x: 5.0, y: 0.0 }, 5.0, 5.0))
    );
    assert_eq!(
        result[3],
        Location::Outside((
            Point { x: 15.0, y: 5.0 },
            15.811388300841896,
            7.0710678118654755
        ))
    );
}


pub fn run(config: RandConfig, bound: Bound, n: usize) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let points = gen_point_list(&mut rng, &config, n);
    let result = locate_n_point(&bound, &points);
    let svg_content = gen_svg(&config, &result, &bound);

    let mut file = File::create("output.svg")?;
    file.write_all(svg_content.as_bytes())?;
    Ok(())
}