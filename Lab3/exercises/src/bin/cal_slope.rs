pub fn cal_slope(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (y2 - y1) / (x2 - x1)
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 5 {
        println!("USAGE: cal_slope x1 y1 x2 y2");
        std::process::exit(1);
    } else {
        let x1 = args[1].parse::<f64>().unwrap();
        let y1 = args[2].parse::<f64>().unwrap();
        let x2 = args[3].parse::<f64>().unwrap();
        let y2 = args[4].parse::<f64>().unwrap();
        println!("The slope of the line is {}.", cal_slope(x1, y1, x2, y2));
    }
}