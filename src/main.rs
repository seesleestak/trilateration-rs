use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[macro_use]
extern crate clap;
use clap::App;

#[derive(Deserialize, Debug)]
struct Beacon {
    x: f32,
    y: f32,
    d: f32,
}

#[derive(Debug)]
struct Location {
    x: f32,
    y: f32,
}

fn read_beacons_from_file<P: AsRef<Path>>(path: P) -> Result<[Beacon; 3], Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader)?;
    Ok(result)
}

fn calc(b: [Beacon; 3]) -> Location {
    let k: f32 = ((b[0].x).powi(2) + (b[0].y).powi(2)
        - (b[1].x).powi(2)
        - (b[1].y).powi(2)
        - (b[0].d).powi(2)
        + (b[1].d).powi(2))
        / (2.0 * (b[0].y - b[1].y))
        - ((b[0].x).powi(2) + (b[0].y).powi(2)
            - (b[2].x).powi(2)
            - (b[2].y).powi(2)
            - (b[0].d).powi(2)
            + (b[2].d).powi(2))
            / (2.0 * (b[0].y - b[2].y));

    let j: f32 = ((b[2].x - b[0].x) / (b[0].y - b[2].y)) - ((b[1].x - b[0].x) / (b[0].y - b[1].y));

    let x: f32 = k / j;

    let y: f32 = ((b[1].x - b[0].x) / (b[0].y - b[1].y)) * x
        + ((b[0].x).powi(2) + (b[0].y).powi(2)
            - (b[1].x).powi(2)
            - (b[1].y).powi(2)
            - (b[0].d).powi(2)
            + (b[1].d).powi(2))
            / (2.0 * (b[0].y - b[1].y));

    Location { x, y }
}

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_filename = matches.value_of("INPUT").unwrap();

    let beacons = read_beacons_from_file(input_filename).unwrap();

    let location: Location = calc(beacons);
    println!("{:#?}", location);
}
