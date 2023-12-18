use csv::{ReaderBuilder, Trim};
use std::io::Read;
use std::fs::File;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    tag_color: String,
}

fn load_points<R: Read>(rdr: R) -> Vec<Point> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut out_list = vec![];
    for record in reader.records() {
        if let Ok(rec) = record {
            let v1: f64 = rec[0].parse().unwrap();
            let v2: f64 = rec[1].parse().unwrap();
            out_list.push(Point {x: v1, y: v2, tag_color: "#000000".to_string()});
        }
    }
    out_list
}
fn main() {
    let data = "\
    1.0, 2.0\n\
    3.0, 4.0\n\
    ".as_bytes();

    let item_list = load_points(data);
    // let mut wtr = Writer::from_writer(vec![]);
    // for item in &item_list {
    //     let rec = [item.0.to_string(), item.1.to_string()];
    //     wtr.write_record(&rec).unwrap();
    // }
    // let out_data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    // print!("{out_data}");
    // println!("{out_data:?}");
    println!("{:?}", item_list);
    println!("{:?}", data)
}

#[test]
fn test_load_points() {
    let data = "\
    1.0, 2.0\n\
    3.0, 4.0\n\
    5.0, 1.0\n\
    ".as_bytes();
    let points = load_points(data);

        assert_eq!(points.len(), 3);
        assert_eq!(points[0].x, 1.0);
        assert_eq!(points[0].y, 2.0);
        assert_eq!(points[0].tag_color, "#000000");

        assert_eq!(points[1].x, 3.0);
        assert_eq!(points[1].y, 4.0);
        assert_eq!(points[1].tag_color,"#000000");

        assert_eq!(points[2].x, 5.0);
        assert_eq!(points[2].y, 1.0);
        assert_eq!(points[2].tag_color, "#000000");
}