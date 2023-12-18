use std:: {
    error::Error,
    io::{ Read, Write},
    path::PathBuf,
};

use clap::{App, Arg};
use csv::{ReaderBuilder, Writer};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point {
    x: f64,
    y: f64,
    color: String,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y, 
            color: "#000000".to_string()
        }
    }

    pub fn new_with_color(x: f64, y: f64, color: String) -> Self {
        Self { x, y, color}
    }

    pub fn dst(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub  fn tag_points(pt_list: &[Point]) -> Vec<Point> {
    let mut result = Vec::new();

    for mut point in pt_list.iter().cloned() {
        if point.dst() > 1.0 {
            point.color = "#80FF8080".to_string();
        } else {
            point.color = "FF808080".to_string();
        } result.push(point)
    } result
}

pub fn save_points<W: Write>(wtr: W, pt_list: &[Point]) {
    let mut wtr = Writer::from_writer(wtr);

    for point in pt_list {
        wtr.write_record(&[ point.x.to_string(), point.y.to_string(), point.color.clone()])
        .expect("should write a record");
    };
    wtr.flush().expect("should flush thr writer");
    wtr.into_inner().unwrap();
}

pub fn load_points<R: Read>(rdr: R) -> Vec<Point> {
    // let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(rdr);
    
    let mut points = vec![];

    for result in rdr.records() {
        if let Ok(record) = result {
            let x1: f64 = record[0].parse().unwrap();
            let y1: f64 = record[1].parse().unwrap();
            // let tag: String = record[2].parse()?;
            points.push( Point::new(x1, y1));
        }
    }
    points
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Input {
    File(PathBuf),
    Stdin,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    pub in_file: Input,
    pub out_file: PathBuf,
}

impl Config {
    pub fn get_args() -> Result<Self, Box<dyn Error>> {
        let matches
            = App::new("point_record")
                .about("Reads points from a CSV file and saves them to another CSV file.")
                .arg(
                    Arg::with_name("in_file")
                    .value_name("IN_FILE")
                    .takes_value(true)
                    .default_value("-")
                    .help("Input file")
                )
                .arg(
                    Arg::with_name("out_file")
                    .value_name("OUT_FILE")
                    .takes_value(true)
                    .help("Output file")
                )
                .get_matches();
        
            let in_file = match matches.value_of("in_file").ok_or("`input` expected")?{ 
                "-" => Input::Stdin,
                path => Input::File(path.into()),
            };

            let out_file = PathBuf::from(matches.value_of("out_file").ok_or("`output`, expected")?);
    
        Ok(Config { in_file, out_file }) 
    }
}
