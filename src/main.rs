use std::{env, error::Error};

use cchuff::run;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(path) => run(path)?,
        None => panic!("File path required"),
    }

    Ok(())
}
