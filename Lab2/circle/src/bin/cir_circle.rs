fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let x_arg = if args.len() > 1 { &args[1] } else { "0" };
    let x = x_arg.parse::<f64>().unwrap_or(0.0);
    println!("the circumference of a circle with radius {} is {}", x, 2.0 * std::f64::consts::PI * x);
}