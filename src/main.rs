use std::{env, error::Error};

use cchuff::run;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(in_path) => match args.next() {
            Some(out_path) => run(&in_path, &out_path)?,
            None => panic!("Output file path required"),
        },
        None => panic!("Input file path required"),
    }

    Ok(())
}
