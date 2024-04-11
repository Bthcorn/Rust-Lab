use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use std::error::Error;

#[test]
fn test_make_xpm2() {
    let ctable = &[
        ("#".into(), "#000000".into()),
        ("-".into(), "#FFFFFF".into()),
    ];
    let rows = ["##--", "##--", "--##", "--##"];
    let pixels: Vec<_> = rows.iter().map(|r| r.to_string()).collect();
    let img = make_xpm2(ctable, &pixels);
    assert_eq!(
        img.colors,
        [
            ("#".into(), "#000000".into()),
            ("-".into(), "#FFFFFF".into())
        ]
    );
    assert_eq!(img.pixels.len(), 4);
    assert_eq!(img.pixels.iter().map(|r| r.len()).max(), Some(4));
    assert_eq!(img.colors.len(), 2);
}

#[derive(Debug, PartialEq)]
pub struct Xpm2 {
    colors: Vec<(String, String)>,
    pixels: Vec<String>,
}

pub fn make_xpm2(ctable: &[(&str, &str)], pixels: &[String]) -> Xpm2 {
    let colors = ctable.iter().map(|(c, hex)| (c.to_string(), hex.to_string())).collect();
    let pixels = pixels.iter().map(|r| r.to_string()).collect();
    Xpm2 { colors, pixels }
}

pub fn read_xpm2(reader: &mut &[u8]) -> Result<Xpm2, Box<dyn Error>> {
    let mut lines = reader.lines().map(|l| l.unwrap());
    let header = lines.next().unwrap();
    if !header.starts_with("! XPM2") {
        panic!("Invalid XPM2 file format");
    }

    let dimensions: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let width = dimensions[0];
    let height = dimensions[1];
    let num_colors = dimensions[2];

    let mut colors = Vec::new();
    for _ in 0..num_colors {
        let color_line = lines.next().unwrap();
        let parts: Vec<&str> = color_line.split_whitespace().collect();
        if parts.len() == 3 {
            let symbol = parts[0].to_string();
            let color = parts[2].to_string();
            colors.push((symbol, color));
        } else {
            panic!("Invalid color definition");
        }
    }

    let pixels: Vec<String> = lines.collect();

    Ok(Xpm2{ colors, pixels })
}

#[test]
fn test_read_xpm2() {
    let checker = "\
    ! XPM2\n\
    4 4 2 1\n\
    # c #000000\n\
    - c #FFFFFF\n\
    ##--\n\
    ##--\n\
    --##\n\
    --##\n\
    ";
    let data = checker.as_bytes().to_vec();
    let mut reader = data.as_slice();
    let img = read_xpm2(&mut reader).unwrap();
    assert_eq!(
        img.colors,
        [
            ("#".into(), "#000000".into()),
            ("-".into(), "#FFFFFF".into())
        ]
    );
    assert_eq!(img.pixels[0].len(), 4);
    assert_eq!(img.pixels.len(), 4);
    assert_eq!(img.colors.len(), 2);
    assert_eq!(img.pixels.iter().map(|r| r.len()).max(), Some(4));
}

pub fn write_to_svg(xpm2: &Xpm2, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    writeln!(file, "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{}\" height=\"{}\">", xpm2.pixels[0].len(), xpm2.pixels.len())?;
    for (i, row) in xpm2.pixels.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            let color = xpm2.colors.iter().find(|(s, _)| s == &c.to_string()).unwrap().1.as_str();
            writeln!(file, "<rect x=\"{}\" y=\"{}\" width=\"1\" height=\"1\" fill=\"{}\"/>", j, i, color)?;
        }
    }
    writeln!(file, "</svg>")?;
    Ok(())
}

pub fn write_to_svg_pixel(xpm2: &Xpm2, pixel_size: usize, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    writeln!(file, "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{}\" height=\"{}\">", xpm2.pixels[0].len() * pixel_size, xpm2.pixels.len() * pixel_size)?;
    for (i, row) in xpm2.pixels.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            let color = xpm2.colors.iter().find(|(s, _)| s == &c.to_string()).unwrap().1.as_str();
            writeln!(file, "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>", j * pixel_size, i * pixel_size, pixel_size, pixel_size, color)?;
        }
    }
    writeln!(file, "</svg>")?;
    Ok(())
}
