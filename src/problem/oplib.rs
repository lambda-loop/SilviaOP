//

const OPLIB_PATH: &'static str = "OPLib";
use super::*;

use std::collections::HashSet;

use std::fs;

pub fn test() {
    let oplib = fs::read_dir(OPLIB_PATH)
        .expect("wrong path to OPLib");

    let mut contents = Vec::new();
    let mut headers = HashSet::new();
    for entry in oplib {
        let entry = entry.unwrap();
        let filename = entry.file_name();
        if !filename.to_string_lossy().ends_with(".oplib") { continue  }
        let filepath = entry.path();

        let content = fs::read_to_string(filepath)
            .expect("problem with contents in {filename:?}");

        let mut lines = content.lines();
        let header = "EDGE_WEIGHT_TYPE"; // : E
        let depots = "DEPOT_SECTION";
        // EDGE_WEIGHT_TYPE : EUC_2D

        while let Some(line) = lines.next() {
            if line.starts_with(header) {
                let (_, end) = line.split_once(": ").unwrap();
                let end = end.trim();
                headers.insert(end.to_owned());
            }

            if line.starts_with(depots) {
                break;
            }
        }

        let mut values = Vec::new();
        while let Some(line) = lines.next() {
            match line.trim().parse::<i32>() {
                Err(_) => break,//()println!("{}", values.len()),
                Ok(num) => {
                    values.push(num);
                },
            }
        }

        if values.len() != 2 {
            println!("diff de 2!");
        } else {
            for v in values {
                if v != 1 && v != -1 {
                    println!("{v}")
                }
            }
            
        }

        contents.push(content);
    }

    for h in headers {
        println!("{h}");
    }
    
    println!("{}", contents[0]);
}



#[derive(Debug)]
struct Point {
    x: f64, 
    y: f64,
}


// AI GENERATED TO MATCH THE REPOSITORY IMPLEMENTATION:

fn euc_2d(a: &Point, b: &Point) -> f64 {
    let xd = a.x - b.x;
    let yd = a.y - b.y;

    ((xd * xd + yd * yd).sqrt() + 0.5).trunc()
}

fn ceil_2d(a: &Point, b: &Point) -> f64 {
    let xd = a.x - b.x;
    let yd = a.y - b.y;

    (xd * xd + yd * yd).sqrt().ceil()
}

fn att(a: &Point, b: &Point) -> f64 {
    let xd = a.x - b.x;
    let yd = a.y - b.y;

    let rij = ((xd * xd + yd * yd) / 10.0).sqrt();
    let tij = rij.trunc();

    if tij < rij {
        tij + 1.0
    } else {
        tij
    }
}


fn geo(a: &Point, b: &Point) -> f64 {
    const PI: f64 = 3.141592;
    const RRR: f64 = 6378.388;

    fn to_radians(coord: f64) -> f64 {
        let deg = coord.trunc();
        let min = coord - deg;

        PI * (deg + 5.0 * min / 3.0) / 180.0
    }

    let latitude_a = to_radians(a.x);
    let longitude_a = to_radians(a.y);

    let latitude_b = to_radians(b.x);
    let longitude_b = to_radians(b.y);

    let q1 = (longitude_a - longitude_b).cos();
    let q2 = (latitude_a - latitude_b).cos();
    let q3 = (latitude_a + latitude_b).cos();

    (
        RRR
            * (
                0.5
                    * (
                        (1.0 + q1) * q2
                            - (1.0 - q1) * q3
                    )
            ).acos()
            + 1.0
    ).trunc()
}
