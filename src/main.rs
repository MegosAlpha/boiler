//! # Boiler - Multi-language Code Preprocessor & Module Loader
//! It's like webpack for everything! It grabs a file with the name
//! specified in whatever is being boiled's 'recipe' section.
//! Checks the same folder for a .boil, then the global folder for a .boil.
//! Copyright (c) 2017 Noah Walker

use std::io;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

/// Returns name with .boiled before actual extension
/// # Examples
/// ```assert_eq!('example.boiled.secret.txt', boiled)```
fn get_boiled_name(filepath: &str) -> String {
    let (fp1, fp2) = filepath.split_at(filepath.find('.').unwrap());
    fp1.to_owned() + ".boiled" + fp2
}

/// This is better than just boil() - it outputs an embeddable string required for metaboiling.
fn metaboil_from_shop(f: &mut File) -> Result<String, io::Error> {
    let mut strbuf = String::new();
    f.read_to_string(&mut strbuf)?;
    Ok(boil_data(&strbuf)?
           .iter()
           .map(|x| x.trim())
           .collect::<Vec<_>>()
           .join("\n") + "\n")
}

/// 'Shop' for 'Ingredients' - Get the replacements through known paths.
fn shop(filepath: &str) -> Result<String, io::Error> {
    match File::open(filepath.to_owned() + ".boil") {
        Ok(mut e) => metaboil_from_shop(&mut e),
        Err(_) => {
            match env::home_dir() {
                Some(path) => {
                    match path.to_str() {
                        Some(e) => {
                            match File::open(e.to_owned() + "/.boiler/" + filepath + ".boil") {
                                Ok(mut f) => metaboil_from_shop(&mut f),
                                Err(_) => {
                                    match File::open(filepath) {
                                        Ok(mut f) => metaboil_from_shop(&mut f),
                                        Err(e) => {
                                            Err(io::Error::new(e.kind(),
                                                               format!("Error with {}: {}",
                                                                       filepath,
                                                                       e)))
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                            Err(io::Error::new(io::ErrorKind::NotFound,
                                               "Your home won't coerce to a string :("))
                        }
                    }
                }
                None => {
                    Err(io::Error::new(io::ErrorKind::NotFound,
                                       "According to our calculations, you have no home directory."))
                }
            }
        }
    }
}

/// Perform line by line boiling.
fn boil_data(secret: &String) -> Result<Vec<String>, io::Error> {
    let lns = secret.lines();
    let mut to_edit = vec![];
    for (ln, line) in lns.clone().enumerate() {
        // We found something!
        if line.len() > 0 {
            if line.split_whitespace().nth(0).unwrap() == "boil" {
                to_edit.push(ln);
            }
        }
    }
    let mut to_write = lns.map(|x| x.to_string() + "\n").collect::<Vec<String>>();
    for ln in to_edit {
        to_write[ln] = shop(to_write[ln].split_whitespace().nth(1).unwrap())?;
    }
    Ok(to_write)
}

/// Boil a file.
fn boil(argfile: String) -> Result<(), io::Error> {
    println!("Boiling {}", &argfile);
    let mut f = File::open(&argfile)?;
    let mut secret = String::new();
    f.read_to_string(&mut secret)?;
    let to_write = boil_data(&secret)?;
    let mut nf = File::create(get_boiled_name(&argfile))?;
    for line in to_write {
        nf.write(line.as_bytes())?;
    }
    Ok(())
}

/// Boils all the files in a list of files.
fn batch_boil(fl: &mut File) -> io::Result<()> {
    let mut bufstr = String::new();
    fl.read_to_string(&mut bufstr)?;
    for ln in bufstr.lines() {
        match boil(String::from(ln)) {
            Ok(()) => println!("Boiling successful!"),
            Err(e) => println!("Oh no, the recipe didn't turn out alright - {}", e),
        }
    }
    Ok(())
}

fn main() {
    println!("Boiler version 1.2");
    if env::args().len() > 1 {
        match boil(env::args().nth(1).unwrap()) {
            Ok(()) => println!("Boiling successful!"),
            Err(e) => println!("Oh no, the recipe didn't turn out alright - {}", e),
        }
    } else {
        match File::open("boiler.files.txt") {
            Ok(mut f) => {
                match batch_boil(&mut f) {
                    Err(e) => println!("Filelist error! {}", e),
                    Ok(()) => println!("Batch completed."),
                }
            }
            Err(_) => println!("Nothing to boil."),
        }
    }
}
