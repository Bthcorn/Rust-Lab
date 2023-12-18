use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    tag_color: String
}


fn main() {
    match load_points() {
        Ok(points) => {
            println!("{:?}", points);
        }
        Err(err) => {
            println!("Error {}", err);
            process::exit(1);
        }
    }
}

// fn load_point()
fn load_points() -> Result<Vec<Point>, Box<dyn Error>> {
    let file_path = get_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(file);
    
    let mut points = vec![];

    for result in rdr.records() {
        if let Ok(record) = result {
            let x1: f64 = record[0].parse()?;
            let y1: f64 = record[1].parse()?;
            points.push( Point{ x: x1, y: y1, tag_color: "#000000".to_string()});
        }
    }
    Ok(points)
}


// fn get_arg()
fn get_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 2 argument, but none")),
        Some(file_path) => Ok(file_path),
    }
}