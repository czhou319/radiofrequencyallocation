use std::io::BufRead;
use std::error::Error;
use std::process;
use std::fs::File;

use serde::Deserialize;

#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Record {
    id: String,
    latitude: Option<f32>,
    longitude: Option<f32>,
}

fn example(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path).expect("Could not open file");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let _record: Record = result?;
    }
    Ok(())
}

pub fn check_csv(path: &str) {
    if let Err(err) = example(path) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

pub fn read_file(path: &str) -> Vec<(u32, f32, f32)> {
    let mut result = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(",").collect();
        if v == ["id", "latitude", "longitude"] {
            continue;
        }
        let x = v[0].parse::<u32>().unwrap();
        let y = v[1].parse::<f32>().unwrap();
        let z = v[2].parse::<f32>().unwrap();
        result.push((x, y, z));    
    }  
    return result;
}
    
    
