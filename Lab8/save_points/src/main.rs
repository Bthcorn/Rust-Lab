use std::{
    env,
    error::Error,
    ffi::OsString,
    process,
    fs::File,
};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    tag_color: String
}

fn save_points(pt_list: Vec<Point>) -> Result<(), Box<dyn Error>> {
    let file_path = get_arg()?;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(file_path)?;

    for point in pt_list {
        wtr.write_record(&[(point.x).to_string(), (point.y).to_string(), (point.tag_color).to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

fn get_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        // None => Err(From::from("expected 1 argument, but got none")),
        None => {
            return "output.csv".parse::<OsString>().map_err(|e| e.into());
        }
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    let points = vec![
        Point {
            x: 1.0,
            y: 2.0,
            tag_color: "#FF808080".to_string(),
        },
        Point {
            x: 3.0,
            y: 4.0,
            tag_color: "#80FF8080".to_string(),
        },
    ];
    if let Err(err) = save_points(points) {
        println!("{}", err);
        process::exit(1);
    } else {
        println!("Completes output!")
    }
}

#[test]
fn test_save_points() {
    let points = vec![
        Point {
            x: 1.0,
            y: 2.0,
            tag_color: "#FF808080".to_string(),
        },
        Point {
            x: 3.0,
            y: 4.0,
            tag_color: "#80FF8080".to_string(),
        },
    ];
    assert!(save_points(points).is_ok());

    let load_result = std::fs::read_to_string(&"./output.csv").unwrap();
    let expected_result = "1,2,#FF808080\n3,4,#80FF8080\n";
    assert_eq!(load_result, expected_result);
}