use std::{
    error::Error,
    fs::File,
    io::{self, Read, BufReader, Write, BufRead},
    path::PathBuf,
};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;
// for main one

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
}

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    // tag_color: String,
}

#[derive(Debug)]
pub struct TagPoint {
    x: f64,
    y: f64,
    tag: String,
}


pub fn get_args() -> MyResult<Config> {
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
    
    Ok(Config {
        in_file: matches.value_of_lossy("in_file").map(Into::into).unwrap(),
        out_file: matches.value_of("out_file").map(|v| v.to_string()),
    }) 
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Determine the input source (stdin or file)
    let in_reader: Box<dyn BufRead> = open(&config.in_file)?;
    // Load points from the input source
    let points = load_points(in_reader)?;
    let tag_points = tag_points1(points);
    // add match to match and run 
    // Determine the output destination (stdout or file)
    // let file = File::create(&config.out_file)?;
    let mut out_writer: Box<dyn Write> = match config.out_file {
        Some(file_path) => {
            let file = File::create(&file_path)?;
            Box::new(file) as Box<dyn Write>
        }
        None => Box::new(std::io::stdout()) as Box<dyn Write>,
    };

    // Save points to the output destination
    save_points(&mut out_writer, tag_points)?;
    Ok(())
}

fn tag_points1(pt_list: Vec<Point>) -> Vec<TagPoint> {
    let mut tag_list = Vec::new();

    for point in pt_list {
        if ((point.x).powf(2.) + (point.y).powf(2.)).powf(0.5) <= 1.0 {
            tag_list.push(TagPoint {
                x: point.x,
                y: point.y,
                tag: "#80FF8080".to_string(),
            });
        } else {
            tag_list.push(TagPoint {
                x: point.x,
                y: point.y,
                tag: "#FF808080".to_string(),
            });
        }
    }
    tag_list
}

// load point one by reading from file
pub fn load_points<R: Read>(rdr: R) -> Result<Vec<Point>, Box<dyn Error>> {
    // let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(rdr);
    
    let mut points = vec![];

    for result in rdr.records() {
        if let Ok(record) = result {
            let x1: f64 = record[0].parse()?;
            let y1: f64 = record[1].parse()?;
            // let tag: String = record[2].parse()?;
            points.push( Point{ x: x1, y: y1} );
        }
    }
    Ok(points)
}


// save point one by writing to file whether creating new one or using existing one
pub fn save_points<W: Write>(wtr: W, pt_list: Vec<TagPoint>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(wtr);

    for point in pt_list {
        wtr.write_record(&[(point.x).to_string(), (point.y).to_string(), point.tag.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}