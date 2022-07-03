use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let marker = &args[1];
    let wrapper_file = &args[2];

    let contents = fs::File::open(wrapper_file)
        .expect("Something went wrong reading the file");
    let wrapper_lines = io::BufReader::new(contents).lines();
    for wline in wrapper_lines {
        if let Ok(wl) = wline {
            if wl.contains(marker) {
                let lines = io::stdin().lock().lines();
                for line in lines {
                    let line = line.expect("Could not read line from standard in");
                    println!("{}", line);
                }
            } else {
                println!("{}", wl);
            }
        }
    }
}
