use categorize_point;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 10 {
        eprintln!("Usage: {} <output_file>", args[0]);
        std::process::exit(1);
    }

    let x  = args[1].parse::<f64>().unwrap();
    let y  = args[2].parse::<f64>().unwrap();
    let cx1 = args[3].parse::<f64>().unwrap();
    let cy1 = args[4].parse::<f64>().unwrap();
    let r1  = args[5].parse::<f64>().unwrap();
    let cx2 = args[6].parse::<f64>().unwrap();
    let cy2 = args[7].parse::<f64>().unwrap();
    let r2  = args[8].parse::<f64>().unwrap();
    let n = args[9].parse::<usize>().unwrap();

    let rand_config = categorize_point::RandConfig {
        x_min: -x,
        x_max: x,
        y_min: -y,
        y_max: y,
    };

    let bound = categorize_point::Bound {
        circle1: categorize_point::Circle {
            center: categorize_point::Point { x: cx1, y: cy1 },
            radius: r1,
        },
        circle2: categorize_point::Circle {
            center: categorize_point::Point { x: cx2, y: cy2 },
            radius: r2,
        },
    };
    categorize_point::run(rand_config, bound, n).unwrap();
}
