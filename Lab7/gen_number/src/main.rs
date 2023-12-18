use rand::Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_arg: &str = if args.len() < 2 { "" } else { &args[1] };
    let n: i32 = match n_arg.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a correct value.");
            return;
        }
    };

    let mut count = 0;
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    for _i in 0..n {
        let x = rng.gen_range(-10.0..=10.0);
        if x >= -1.0 && x <= 1.0 {
            count += 1
        }
        result.push(x);
    }
    println!("{:?}", result);
    println!("The probability is {}", count as f32 / n as f32);
}
