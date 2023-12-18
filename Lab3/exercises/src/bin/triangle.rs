fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let n_arg = if args.len() > 1 { &args[1] } else { "0" };
    let n = n_arg.parse::<i32>().unwrap_or(0);

    for _i in 0..n {
        for _j in 0.._i+1 {
            print!("*");
        }
        println!();
    }
}