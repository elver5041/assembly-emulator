use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

use crate::commands::commands::Commands;
mod commands;

const ANSI_BOLD: &str = "\x1b[32m";
const ANSI_GREEN: &str = "\x1b[1m";
const ANSI_RESET: &str = "\x1b[0m";

fn main() {
    const VERBOSE: bool = true;
    println!("Starting compilation");
    let mut timer: Instant = Instant::now();
    let start: Instant = Instant::now();
    let file: File = File::open("idk.txt").expect("input file");
    let lines: Vec<String> = match read_lines(file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read input: {}", e);
            ::std::process::exit(-1);
        }
    };
    if VERBOSE {
        println!("     {ANSI_BOLD}{ANSI_GREEN}Reading{ANSI_RESET} file in {:.2}s", start.elapsed().as_millis() as f32/1000.0);
        timer = Instant::now();
    }
    let separated: Vec<Vec<String>> = separator(lines);
    let (tokenized, headers) = new_func(separated);
    if VERBOSE {
        println!("     {ANSI_BOLD}{ANSI_GREEN}Reading{ANSI_RESET} file in {:.2}s", timer.elapsed().as_millis() as f32/1000.0);
        timer = Instant::now();
    }
    dbg!(tokenized);
    dbg!(headers);

}

fn read_lines(file: File) -> Result<Vec<String>, Error> {
    Ok(
        BufReader::new(file).lines()
            .collect::<Result<Vec<String>, Error>>()?
            //.into_iter()
            //.filter(|v| !v.is_empty())
            //.collect()
    )
}

fn separator(input: Vec<String>) -> Vec<Vec<String>> {
    input.into_iter()
        .map(|item| 
            item.split(char::is_whitespace)
            .map(|s| s.to_string())
            //.filter(|c| c!="")
            .collect())
        .collect()
}

fn new_func(input: Vec<Vec<String>>) -> (Vec<Commands>, HashMap<String,usize>){
    for line in input{
        
    }
    (vec![],HashMap::new())
}