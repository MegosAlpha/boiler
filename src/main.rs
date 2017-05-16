//! # Boiler - Multi-language Code Preprocessor & Module Loader
//! It's like webpack for everything! It grabs a file with the name
//! specified in whatever is being boiled's 'recipe' section.
//! Copyright (c) 2017 Noah Walker


use std::io;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

/// 'Shop' for 'Ingredients' - Get the replacements through known paths.
fn shop(filepath: &str) -> Result<File, io::Error> {
    match File::open(filepath.to_owned() + ".boil") {
        Ok(e) => Ok(e),
        Err(_) => {
            match env::home_dir() {
                Some(path) => match path.to_str() {
                    Some(e) => File::open(e.to_owned() + "/.boiler/" + filepath + ".boil"),
                    None => Err(io::Error::new(io::ErrorKind::NotFound, "Your home won't coerce to a string :("))
                },
                None => Err(io::Error::new(io::ErrorKind::NotFound, "According to our calculations, you have no home directory.")),
            }
        }
    }
}

/// Perform line by line boiling. Probably not the best code ever.
/// Just pass an argument to it.
fn boil() -> Result<(), io::Error> {
    let argfile = env::args().nth(1).unwrap();
    let mut f = File::open(&argfile)?;
    let mut secret = String::new();
    f.read_to_string(&mut secret)?;
    let lns = secret.lines();
    let mut to_edit = vec![];
    for (ln, line) in lns.clone().enumerate() {
        // We found something!
        if line.split_whitespace().nth(0).unwrap() == "boil" {
            to_edit.push(ln);
        }
    }
    let mut to_write = lns.map(|x| x.to_string() + "\n").collect::<Vec<String>>();
    for ln in to_edit {
        let mut shopper = String::new();
        shop(to_write[ln].split_whitespace().nth(1).unwrap())?.read_to_string(&mut shopper)?;
        to_write[ln] = shopper.as_str().to_owned();
    }
    let mut nf = File::create(env::args().nth(1).unwrap() + ".boiled")?;
    for line in to_write {
        nf.write(line.as_bytes())?;
    }
    Ok(())
}

fn main() {
    match boil() {
        Ok(()) => println!("Boiling successful!"),
        Err(e) => println!("Oh no, the recipe didn't turn out alright - {}", e),
    }
}
