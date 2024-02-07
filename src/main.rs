use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::iter::zip;
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
        println!("     {ANSI_BOLD}{ANSI_GREEN}Reading{ANSI_RESET} file in {:.2}s", timer.elapsed().as_millis() as f32/1000.0);
        timer = Instant::now();
    }
    let separated: Vec<Vec<String>> = separator(lines);
    let tokenized = match new_func(separated) {
        Ok(s) => s,
        Err(e) => {
            println!("{ANSI_BOLD}{ANSI_RED}Failed to parse input{ANSI_RESET}: {}", e);
            ::std::process::exit(2);
        }
    }; 
    if VERBOSE {
        println!("     {ANSI_BOLD}{ANSI_GREEN}Parsing{ANSI_RESET} file in {:.2}s", timer.elapsed().as_millis() as f32/1000.0);
        //timer = Instant::now();
    }
    println!("{ANSI_BOLD}{ANSI_GREEN}Compilation complete{ANSI_RESET}: file in {:.2}s", start.elapsed().as_millis() as f32/1000.0);

    println!("{:?}",tokenized);

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

fn new_func(input: Vec<Vec<String>>) -> Result<Vec<Commands>,TokenError>{
    const FUNCTION_INTRO:char = '.';
    const  COMMENT_INTRO:char = '#';
    let mut fncounter = 0;
    let mut next_func: bool = false;
    let mut l:usize=0;
    let mut token: Commands = Commands::NOP;
    let mut tokens: Vec<Commands> = Vec::new();
    let mut funjumps: Vec<String> = Vec::new();
    let mut functions: Vec<(String, usize)> = Vec::new();
    let mut calls: Vec<usize> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        //i+1
        if next_func {
            return Err(TokenError { error_type: Errors::FunctionNotThere, line: (i+1,0)});
        }
        if line.len()==1 && line[0]=="" {
            continue;
        }
        let mut counter: usize = 0;
        for word in line {
            if word == &COMMENT_INTRO.to_string() {
                break;
            }
            if word == "" {
                counter+=1;
                continue;
            } 
            if next_func {
                token = Commands::FUN;
                if functions.iter().any(|(s,_)| s == &word[1..]) {
                    return Err(TokenError { error_type: Errors::FunctionAlreadyInUse(word[1..].to_string()), line: (i+1,counter) })
                }
                functions.push((word.to_string(), fncounter));
                next_func=false;
            }
            if token != Commands::NOP {
                match token {
                    Commands::ADD(_) => {
                        match word.parse::<usize>().ok() {
                            Some(n) => tokens.push(Commands::ADD(n)),
                            None => return Err(TokenError { error_type: Errors::InvalidContinuation(token, word.to_owned()), line: (i+1,counter)}),
                        }
                    },
                    Commands::SUB(_) => {
                        match word.parse::<usize>().ok() {
                            Some(n) => tokens.push(Commands::SUB(n)),
                            None => return Err(TokenError { error_type: Errors::InvalidContinuation(token, word.to_owned()), line: (i+1,counter)}),
                        }
                    },
                    Commands::LDN(_) => {
                        match word.parse::<usize>().ok() {
                            Some(n) => tokens.push(Commands::LDN(n)),
                            None => return Err(TokenError { error_type: Errors::InvalidContinuation(token, word.to_owned()), line: (i+1,counter)}),
                        }
                    },
                    Commands::LDA(_) => {
                        match word.parse::<usize>().ok() {
                            Some(n) => tokens.push(Commands::LDA(n)),
                            None => return Err(TokenError { error_type: Errors::InvalidContinuation(token, word.to_owned()), line: (i+1,counter)}),
                        }
                    },
                    Commands::JMP(_) => {
                        calls.push(fncounter);
                        funjumps.push(word.to_string());
                        tokens.push(Commands::JMP(funjumps.len()-1))
                    },
                    Commands::JNZ(_) => {
                        calls.push(fncounter);
                        funjumps.push(word.to_string());
                        tokens.push(Commands::JNZ(funjumps.len()-1))
                    },
                    Commands::FUN => {
                        tokens.push(token);
                    },
                    Commands::NOP | Commands::SAV | Commands::SWP => ()
                }
                token = Commands::NOP;
                fncounter+=1;
            } 
            else if word.starts_with(COMMENT_INTRO){
                break;
            }
            else if word.starts_with(FUNCTION_INTRO) {
                if word[1..] == "".to_owned() {
                    next_func = true;
                    continue;
                }
                if functions.iter().any(|(s,_)| s == &word[1..]) {
                    return Err(TokenError { error_type: Errors::FunctionAlreadyInUse(word[1..].to_string()), line: (i+1,counter) })
                }
                functions.push((word[1..].to_string(), functions.len()));
                next_func = false;
                tokens.push(Commands::FUN);
                fncounter +=1;
                continue;
            } 
            else {
                token = match word.to_ascii_uppercase().as_str() {
                    "ADD" => Commands::ADD(0),
                    "SUB" => Commands::SUB(0),
                    "JMP" => Commands::JMP(0),
                    "JNZ" => Commands::JNZ(0),
                    "SAV" => Commands::SAV,
                    "SWP" => Commands::SWP,
                    "LDN" => Commands::LDN(0),
                    "LDA" => Commands::LDA(0),
                    _ => return Err(TokenError { error_type: Errors::InvalidToken(word.to_owned()), line: (i+1,counter) })
                };
                match token {
                    Commands::ADD(_) | Commands::SUB(_) | Commands::JNZ(_) | Commands::JMP(_) |
                    Commands::LDA(_) | Commands::LDN(_)=> (),
                    Commands::SAV | Commands::SWP => {
                        tokens.push(token);
                        token = Commands::NOP;
                    }
                    Commands::NOP | Commands::FUN => ()
                }
            }
            
            counter += word.len()+1;
        }
        l=i;
    }
    
    if token != Commands::NOP {
        match token {
            Commands::FUN | Commands::SAV | Commands::SWP => {
                tokens.push(token);
            },
            _ => todo!()
        }
    }

    if next_func {
        return Err(TokenError { error_type: Errors::FunctionNotThere, line: (l+2,0)});
    }

    for (i,s)  in zip(calls, funjumps) {
        match functions.iter().filter(|(a,_)| a==&s).collect::<Vec<&(String,usize)>>().pop() {
            Some((_,y)) => {
                tokens[i] = match tokens[i] {
                    Commands::JNZ(_) => Commands::JNZ(*y),
                    _ => panic!("wtf? fixing error?")
                }
            }
            None => return Err(TokenError { error_type: Errors::FunctionDoesntExist(s), line: (i,0) })
        };
    }
    Ok(tokens)
}