mod json;
mod openslide;

use std::env;
use serde::{Deserialize, Serialize};

// TODO: Move to data file
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    predictions: Vec<u64>,
    coordinates: Vec<Vec<u64>>,
}

fn main() {
    // TODO: To be replaced
    let json_file_path = env::current_dir().unwrap().ancestors().nth(2).unwrap().join("plugins/tiatoolbox/patch_prediction/0.raw.json");
    let tiff_file_path = env::current_dir().unwrap().ancestors().nth(1).unwrap().join("images/image.tiff");
    let svs_file_path = env::current_dir().unwrap().ancestors().nth(1).unwrap().join("images/image.svs");

    match json::parse::<Data>(json_file_path) {
        Ok(d) => println!("Data is: {:?}", d),
        Err(e) => println!("Error: {:?}", e),
    };

    match openslide::decode(tiff_file_path) {
        Ok(d) => println!("TIFF: {:?}", d),
        Err(e) => println!("Error: {:?}", e),
    };

    match openslide::decode(svs_file_path) {
        Ok(d) => println!("SVS: {:?}", d),
        Err(e) => println!("Error: {:?}", e),
    };
}