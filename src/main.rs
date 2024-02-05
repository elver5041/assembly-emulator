use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

mod commands;
use commands::Errors;
use commands::TokenError;
use commands::Commands;


const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";

fn main() {
    const VERBOSE: bool = true;
    println!("Starting compilation");
    let mut timer: Instant = Instant::now();
    let start: Instant = Instant::now();
    let file: File = File::open("idk.txt").expect("input file");
    let lines: Vec<String> = match read_lines(file) {
        Ok(s) => s,
        Err(e) => {
            println!("{ANSI_BOLD}{ANSI_RED}Failed to read input{ANSI_RESET}: {}", e);
            ::std::process::exit(1);
        }
    };
    if VERBOSE {
        println!("     {ANSI_BOLD}{ANSI_GREEN}Reading{ANSI_RESET} file in {:.2}s", start.elapsed().as_millis() as f32/1000.0);
        timer = Instant::now();
    }
    let separated: Vec<Vec<String>> = separator(lines);
    let (tokenized, function_headers) = match new_func(separated) {
        Ok((s,v)) => (s,v),
        Err(e) => {
            println!("{ANSI_BOLD}{ANSI_RED}Failed to parse input{ANSI_RESET}: {}", e);
            ::std::process::exit(2);
        }
    }; 
    if VERBOSE {
        println!("     {ANSI_BOLD}{ANSI_GREEN}Reading{ANSI_RESET} file in {:.2}s", timer.elapsed().as_millis() as f32/1000.0);
        timer = Instant::now();
    }
    dbg!(tokenized);
    dbg!(function_headers);

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

fn new_func(input: Vec<Vec<String>>) -> Result<(Vec<Commands>, HashMap<String,usize>),TokenError>{
    const FUNCTION_INTRO:char = '.';
    let mut fnpointer: usize = 0;
    let mut functions: HashMap<String, usize> = HashMap::new();
    let mut next_func: bool = false;
    let mut l:usize=0;
    let mut token: Commands = Commands::NOP;
    let mut tokens: Vec<(Commands,u8)> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        //i+1
        if next_func {
            return Err(TokenError { error_type: Errors::FunctionNotThere, line: (i+1,0)});
        }
        if line.len()==1 && line[0]=="" {
            continue
        }
        let mut counter: usize = 0;
        for word in line {
            if next_func {
                if word == "" {
                    return Err(TokenError { error_type: Errors::FunctionNotThere, line: (i+1,counter)});
                }
                else {
                    functions.insert(word.to_string(), fnpointer);
                    next_func=false;
                    continue;
                }
            }
            if word == "" {
                counter+=1;
                continue;
            } 
            if token != Commands::NOP {
                if word.len()>2 {
                    let num = match word[2] {
                        "x" => {word[2..].parse()}
                    }
                }
                match token {
                    Commands::ADD => {
                        
                    }
                    Commands::NOP|
                    Commands::JNE => ()
                }
            }
            if word.starts_with(FUNCTION_INTRO) {
                if word[1..] == "".to_owned() {
                    next_func = true;
                    continue;
                }
                functions.insert(word[1..].to_string(), fnpointer);
                next_func=false;
            }
            match word.to_ascii_uppercase().as_str() {
                "ADD" => token = Commands::ADD,
                _ => ()
            }
        }
        l=i;
    }
    if next_func {
        return Err(TokenError { error_type: Errors::FunctionNotThere, line: (l+2,0)});
    }
    Ok((vec![],functions))
}