//

use std::fs;

mod problem;
use problem::Problem;

fn main() {
    println!("Hello, world!");
    let dir = fs::read_dir("data").expect("data dir");
    for entry in dir {
        if let Ok(dir_entry) = entry {
            let path = dir_entry.path();
            if path.extension().is_some_and(|ext| ext == "txt") {
                let input = fs::read_to_string(path).unwrap();
                let p = Problem::new(&input);
                println!("{:?}", p);
            } else {
                println!("{:?}", path);
            }
        }
    }
    // fs::read_to_string("data/");
}
