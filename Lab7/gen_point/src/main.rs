use rand::Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_arg: &str = if args.len() < 2 {""} else {&args[1]};
    let n: i32 =  match n_arg.parse() {
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
        let x: f32 = rng.gen_range(-1.0..=1.0);
        let y: f32 = rng.gen_range(-1.0..=1.0);
        if (x.powf(2.) + y.powf(2.)).powf(0.5) <= 1.0 {
            count += 1
        }
        result.push((x, y));
    }
    let p = count as f32 / n as f32;
    println!("{:?}", result);
    println!("The probability is {}", p);
    println!("P * 4 is {}", p * 4. );

}