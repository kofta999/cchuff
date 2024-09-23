use std::{env, error::Error};

use cchuff::run;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(in_path) => match args.next() {
            Some(out_path) => run(&in_path, Some(&out_path))?,
            None => run(&in_path, None)?,
        },
        None => panic!("Input file path required"),
    }

    Ok(())
}
