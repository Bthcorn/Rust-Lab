use clap::{App, Arg};
use rand::Rng;
use std::{
    error::Error,
    fs::File,
    io::Write,
};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Layer {
    name: String,
    color: String,
    points: Vec<Point>,
}

pub fn gen_layer<R: Rng>(rng: &mut R, name: String, color: String) -> Layer {
    let mut result = Vec::new();
    let mut rng = rand::thread_rng();

    let n = rng.gen_range(20..=50);

    for _ in 0..n {
        let x = rng.gen_range(-100..=100);
        let y = rng.gen_range(-100..=100);

        result.push(Point {x, y});
    }

    Layer {
        name,
        color,
        points: result,
    }
}

pub fn gen_list_layer<R: Rng>(rng: &mut R, n: i32) -> Vec<Layer> {
    let mut result = Vec::new();
    let mut rng = rand::thread_rng();
    
    for i in 1..n+1 {
        let name = format!("Layer {i}");
        let color = random_rgb_color();
        let generated = gen_layer(&mut rng, name, color);
        result.push(generated);
    }

    result 
}

pub fn random_rgb_color() -> String {
    let mut rng = rand::thread_rng();

    let a = rng.gen_range(0..256);
    let b = rng.gen_range(0..256);
    let c = rng.gen_range(0..256);
    let d = rng.gen_range(0..256);

    format!("#{:02X}{:02X}{:02X}{:02X}", a, b, c, d)
}


pub fn save_layers<W: Write>(layers: &[Layer], mut writer: W) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(writer);
    
    for layer in layers {
        for point in &layer.points {
            let name = layer.name.clone();
            let color = layer.color.clone();
            wtr.serialize((name, color, point.x, point.y))?;
        }
    }

    wtr.flush()?;
    Ok(())
}


pub fn generate_svg(layers: Vec<Layer>, output: File) -> Result<(), Box<dyn Error>> {
    let mut output = output;
    writeln!(output, "<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n")?;
    writeln!(output, "<rect width=\"500\" height=\"500\" style=\"fill:rgb(255,255,255)\" />\n")?;

    for layer in layers {
        // writeln!(output, "<g id='{}' fill='{}'>", layer.name, layer.color)?;
        for point in layer.points {
            writeln!(output, "{}", &format!("<circle cx=\"{}\" cy=\"{}\" r=\"10\" fill=\"{}\" />\n", point.x + 250, point.y + 250, layer.color))?;
        }
        
    }
    writeln!(output, "</svg>")?;
    Ok(())
}