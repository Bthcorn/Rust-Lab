use clap::{App, Arg};
use gen_layer::{gen_list_layer, save_layers};
use rand::Rng;
use std::{
    io::Write,
    error::Error,
    fs::File,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("gen_svg")
        .version("0.1.0")
        .author("Your Name")
        .about("Generate SVG files")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Output file")
            .takes_value(true))
        .arg(Arg::with_name("layers")
            .short("l")
            .long("layers")
            .value_name("N")
            .help("Number of layers")
            .takes_value(true))
        .get_matches();

    let output = matches.value_of("output").unwrap_or("output.csv");
    let layers = matches.value_of("layers").unwrap_or("1");

    println!("Output file: {}", output);
    println!("Number of layers: {}", layers);

    let n = layers.parse::<i32>().unwrap();
    let layers = gen_list_layer(&mut rand::thread_rng(), n);
    let output = File::create(output)?;
    let _ = save_layers(&layers, output);

    Ok(())
}
