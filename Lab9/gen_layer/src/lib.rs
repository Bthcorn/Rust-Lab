use rand::Rng;
use std::{
    io::Write,
    error::Error,
    fs::File,
};

use clap::{App, Arg};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_rgb_color() {
        let color = random_rgb_color();
        assert_eq!(color.len(), 9);
    }

    #[test]
    fn test_gen_layer() {
        let mut rng = rand::thread_rng();
        let layer = gen_layer(&mut rng, "Layer 1".to_string(), "#FF0000".to_string());
        // assert_eq!(layer.points.len(), 20);
        assert!(layer.points.len() <= 50 && layer.points.len() >= 20);
        assert!(layer.points.iter().all(|p| p.x >= -100 && p.x <= 100));
    }
}