//! # Boiler - Multi-language Code Preprocessor & Module Loader
//! It's like webpack for everything! It grabs a file with the name
//! specified in whatever is being boiled's 'recipe' section.
//! Checks the same folder for a .boil, then the global folder for a .boil.
//! Copyright (c) 2017 Noah Walker

extern crate regex;
use regex::Regex;
extern crate toml;
use std::io;
use std::env;
use std::fs::{File, metadata, canonicalize};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::path::PathBuf;

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

/// Loads and Parses the configuration.
fn parse_config() -> io::Result<HashMap<String, String>> {
    let mut txt = String::new();
    File::open("boiler.config.toml")?.read_to_string(&mut txt)?;
    match toml::from_str(txt.as_str()) {
        Ok(t) => Ok(t),
        Err(_) => {
            Err(io::Error::new(io::ErrorKind::Other,
                               "The file won't serialize - please check your syntax"))
        }
    }
}

/// Less crazy matching structure in shop
fn home_shop(filepath: &str) -> Result<String, io::Error> {
    match env::home_dir() {
        Some(path) => {
            match path.to_str() {
                Some(e) => {
                    match File::open(e.to_owned() + "/.boiler/" + filepath + ".boil") {
                        Ok(mut f) => metaboil_from_shop(&mut f),
                        Err(e) => Err(e), 
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

/// 'Shop' for 'Ingredients' - Get the replacements through known paths (and in 1.3, config).
fn shop(filepath: &str) -> Result<String, io::Error> {
    match File::open("recipes/".to_owned() + filepath + ".boil") {
        Ok(mut e) => metaboil_from_shop(&mut e),
        Err(_) => {
            match File::open("recipes/".to_owned() + filepath) {
                Ok(mut f) => metaboil_from_shop(&mut f),
                Err(_) => {
                    match File::open(filepath.to_owned() + ".boil") {
                        Ok(mut f) => metaboil_from_shop(&mut f),
                        Err(_) => {
                            match File::open(filepath) {
                                Ok(mut f) => metaboil_from_shop(&mut f),
                                Err(_) => {
                                    match home_shop(filepath) {
                                        Ok(f) => Ok(f),
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
                    }
                }
            }
        }
    }
}

/// Perform regular expression boiling (v1.4) - replaces line-by-line style
fn boil_data (secret: &String) -> Result<Vec<String>, io::Error> {
    let mut ret_secret = secret.clone();
    // Match the file names
    let re = Regex::new(r"\{boil ([\w\d.]+)\}").unwrap();
    // Prepare a set to put all the matches into (eliminating the chances of matches - after all, Boiler likes pure functions when dealing with files)
    let mut boilset = HashMap::new();
    // Parse the configuration, it's sort of important
    let mconfig = match parse_config() {
        Ok(m) => m,
        Err(_) => {
            HashMap::new()
        }
    };
    // Check all the matching capture groups, and boil them. Once boiled, add them if it is unique.
    for caps in re.captures_iter(secret) {
        println!("match found");
        if !boilset.contains_key(&caps[0]) {
            if mconfig.contains_key(&caps[1]) {
                boilset.insert(caps[0].to_string(), match mconfig.get(&caps[1]) {
                    Some(r) => r,
                    None => ""
                }.to_string());
            } else {
                boilset.insert(caps[0].to_string(), match shop(&caps[1]) {
                    Ok(r) => r.trim().to_string(),
                    Err(_) => "".to_string()
                });
            }
        }
    }
    for (key,val) in boilset.iter() {
        println!("{}: {}", key, val);
        ret_secret = ret_secret.replace(key, val);
    }
    //println!("{}", ret_secret);
    return Ok(ret_secret.lines().map(|x| x.to_string() + "\n").collect::<Vec<String>>());
}

fn switchback(printout: &str, resetpath: &PathBuf) -> io::Result<()>{
    println!("{}", printout);
    env::set_current_dir(resetpath)?;
    Ok(())
}

/// Boil a file.
fn boil(argfile: String) -> Result<(), io::Error> {
    let md = metadata(&argfile).unwrap();
    if md.is_dir() {
        println!("Boiling directory {}", argfile);
        let starter = canonicalize(".")?;
        env::set_current_dir(&argfile)?;
        match File::open("boiler.files.txt") {
            Ok(mut f) => {
                match batch_boil(&mut f) {
                    Err(e) => switchback(format!("Filelist error! {}", e).as_str(), &starter)?,
                    Ok(()) => switchback("Batch completed!", &starter)?,
                }
            }
            Err(_) => switchback("Nothing to boil.", &starter)?,
        }
    } else {
        println!("Boiling file {}", argfile);
        let mut f = File::open(&argfile)?;
        let mut secret = String::new();
        f.read_to_string(&mut secret)?;
        let to_write = boil_data(&secret)?;
        let mut nf = File::create(get_boiled_name(&argfile))?;
        for line in to_write {
            nf.write(line.as_bytes())?;
        }
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
    println!("Boiler version 1.3.2");
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
